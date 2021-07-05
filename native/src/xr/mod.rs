use crate::game::graphics::kernel;
use crate::game::system::framework;

use bindings::Windows::Win32::Graphics::Direct3D11::{
    ID3D11Device, ID3D11Texture2D, D3D11_TEXTURE2D_DESC,
};
use bindings::Windows::Win32::Graphics::Dxgi::DXGI_FORMAT_R8G8B8A8_UNORM;
use windows::Abi;

use anyhow;
use once_cell::unsync::OnceCell;
use openxr;

pub const VIEW_COUNT: u32 = 2;
const VIEW_TYPE: openxr::ViewConfigurationType = openxr::ViewConfigurationType::PRIMARY_STEREO;
pub static mut XR_INSTANCE: OnceCell<XR> = OnceCell::new();

pub struct XR {
    instance: openxr::Instance,
    session: openxr::Session<openxr::D3D11>,
    frame_waiter: openxr::FrameWaiter,
    frame_stream: openxr::FrameStream<openxr::D3D11>,
    stage: openxr::Space,

    swapchain: openxr::Swapchain<openxr::D3D11>,
    // buffer_images is used to stage the images before they
    // can be copied to the swapchain. We can't acquire a
    // specific swapchain image, so we have to keep retrying
    // until we handle both.
    buffer_images: Vec<ID3D11Texture2D>,
    swapchain_images: Vec<ID3D11Texture2D>,

    environment_blend_mode: openxr::EnvironmentBlendMode,
    frame_state: Option<openxr::FrameState>,

    window_size: (u32, u32),
}

impl XR {
    pub fn new() -> anyhow::Result<XR> {
        let entry = openxr::Entry::linked();
        let available_extensions = entry.enumerate_extensions()?;
        assert!(available_extensions.khr_d3d11_enable);

        let mut enabled_extensions = openxr::ExtensionSet::default();
        enabled_extensions.khr_d3d11_enable = true;
        let instance = entry.create_instance(
            &openxr::ApplicationInfo {
                application_name: "XIVR",
                application_version: 0,
                engine_name: "XIVR",
                engine_version: 0,
            },
            &enabled_extensions,
            &[],
        )?;

        let instance_props = instance.properties()?;
        log!(
            "loaded OpenXR runtime: {} {}",
            instance_props.runtime_name,
            instance_props.runtime_version
        );

        // Request a form factor from the device (HMD, Handheld, etc.)
        let system = instance.system(openxr::FormFactor::HEAD_MOUNTED_DISPLAY)?;

        // Check what blend mode is valid for this device (opaque vs transparent displays). We'll just
        // take the first one available!
        let environment_blend_mode =
            instance.enumerate_environment_blend_modes(system, VIEW_TYPE)?[0];

        // We don't do anything with reqs as running this version of the game necessitates that you have
        // support for D3D11.
        let _reqs = instance.graphics_requirements::<openxr::D3D11>(system)?;

        let system_properties = instance.system_properties(system)?;
        log!(
            "system properties: {} ({}:{:?}), orientation {}, position {}",
            system_properties.system_name,
            system_properties.vendor_id,
            system_properties.system_id,
            system_properties.tracking_properties.orientation_tracking,
            system_properties.tracking_properties.position_tracking
        );

        let views = instance.enumerate_view_configuration_views(system, VIEW_TYPE)?;
        assert_eq!(
            views[0].recommended_image_rect_width,
            views[1].recommended_image_rect_width
        );
        assert_eq!(
            views[0].recommended_image_rect_height,
            views[1].recommended_image_rect_height
        );
        assert_eq!(
            views[0].recommended_swapchain_sample_count,
            views[1].recommended_swapchain_sample_count
        );

        let window = &framework::Framework::get().window;
        let window_size = window.get_size();
        let new_window_size = (
            views[0].recommended_image_rect_width,
            views[0].recommended_image_rect_height,
        );
        window.set_resizing_enabled(false);
        window.set_size(new_window_size);

        let (session, frame_waiter, frame_stream) = unsafe {
            instance.create_session::<openxr::D3D11>(
                system,
                &openxr::d3d::SessionCreateInfo {
                    device: std::mem::transmute(kernel::Device::get().device.abi()),
                },
            )?
        };

        let stage = session
            .create_reference_space(openxr::ReferenceSpaceType::STAGE, openxr::Posef::IDENTITY)?;

        let mut swapchain = session.create_swapchain(&openxr::SwapchainCreateInfo {
            create_flags: openxr::SwapchainCreateFlags::EMPTY,
            usage_flags: openxr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | openxr::SwapchainUsageFlags::SAMPLED,
            format: DXGI_FORMAT_R8G8B8A8_UNORM.0,
            // The Vulkan graphics pipeline we create is not set up for multisampling,
            // so we hardcode this to 1. If we used a proper multisampling setup, we
            // could set this to `views[0].recommended_swapchain_sample_count`.
            sample_count: 1,
            width: new_window_size.0,
            height: new_window_size.1,
            face_count: 1,
            array_size: VIEW_COUNT,
            mip_count: 1,
        })?;

        let swapchain_images: Vec<ID3D11Texture2D> = swapchain
            .enumerate_images()?
            .iter()
            .map(|x| unsafe { ID3D11Texture2D::from_abi(*x as *mut _) })
            .collect::<Result<Vec<ID3D11Texture2D>, _>>()?;

        let mut desc: D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        {
            let index = swapchain.acquire_image()?;
            swapchain.wait_image(openxr::Duration::INFINITE)?;
            unsafe {
                swapchain_images[index as usize].GetDesc(&mut desc);
            }
            swapchain.release_image()?;
        }

        let mut buffer_images: Vec<_> = vec![];
        for i in 0..VIEW_COUNT {
            let device = &kernel::Device::get().device;
            let texture = unsafe { device.CreateTexture2D(&desc, std::ptr::null())? };
            buffer_images.push(texture);
        }

        Ok(XR {
            instance,
            session,
            frame_waiter,
            frame_stream,
            stage,

            swapchain,
            buffer_images,
            swapchain_images,

            environment_blend_mode,
            frame_state: None,

            window_size,
        })
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        let session = &self.session;
        // TODO: do something with this
        let mut session_running = true;
        let mut event_storage = openxr::EventDataBuffer::new();

        while let Some(event) = self.instance.poll_event(&mut event_storage)? {
            use openxr::Event::*;
            use openxr::SessionState;
            match event {
                SessionStateChanged(e) => {
                    // Session state change is where we can begin and end sessions, as well as
                    // find quit messages!
                    log!("entered state {:?}", e.state());
                    match e.state() {
                        SessionState::READY => {
                            session.begin(VIEW_TYPE)?;
                            session_running = true;
                        }
                        SessionState::STOPPING => {
                            session.end()?;
                            session_running = false;
                        }
                        SessionState::EXITING | SessionState::LOSS_PENDING => {
                            break;
                        }
                        _ => {}
                    }
                }
                InstanceLossPending(_) => {
                    break;
                }
                EventsLost(e) => {
                    log!("lost {} events", e.lost_event_count());
                }
                _ => {}
            }
        }

        self.frame_state = Some(self.frame_waiter.wait()?);
        self.frame_stream.begin()?;

        Ok(())
    }

    pub fn post_update(&mut self) -> anyhow::Result<()> {
        let frame_state = &self
            .frame_state
            .ok_or(anyhow::Error::msg("failed to get frame state"))?;

        self.frame_stream.end(
            frame_state.predicted_display_time,
            self.environment_blend_mode,
            &[],
        )?;

        Ok(())
    }
}

impl Drop for XR {
    fn drop(&mut self) {
        framework::Framework::get()
            .window
            .set_resizing_enabled(true);
        framework::Framework::get()
            .window
            .set_size(self.window_size);
    }
}
