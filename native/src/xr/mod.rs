use crate::ct_config;
use crate::game::graphics::{kernel, render};
use crate::game::system::framework;
use crate::singleton;

use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
use bindings::Windows::Win32::Graphics::Dxgi as dxgi;
use windows::Abi;

use openxr;

pub use crate::ct_config::xr::VIEW_COUNT;
const VIEW_TYPE: openxr::ViewConfigurationType = openxr::ViewConfigurationType::PRIMARY_STEREO;

#[allow(dead_code)]
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
    buffer_images: Vec<d3d::ID3D11Texture2D>,
    buffer_srvs: Vec<d3d::ID3D11ShaderResourceView>,
    swapchain_images: Vec<d3d::ID3D11Texture2D>,

    environment_blend_mode: openxr::EnvironmentBlendMode,
    frame_state: Option<openxr::FrameState>,

    old_window_size: (u32, u32),
    frame_size: (u32, u32),

    instance_properties: openxr::InstanceProperties,
    system_properties: openxr::SystemProperties,
    available_extensions: openxr::ExtensionSet,
}
singleton!(XR);

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
        let instance_properties = instance.properties()?;

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

        let old_window_size = unsafe { framework::Framework::get().window().get_size() };
        let new_window_size = if ct_config::xr::CHANGE_WINDOW_SIZE {
            (
                views[0].recommended_image_rect_width,
                views[0].recommended_image_rect_height,
            )
        } else {
            old_window_size
        };

        let (session, frame_waiter, frame_stream) = unsafe {
            instance.create_session::<openxr::D3D11>(
                system,
                &openxr::d3d::SessionCreateInfo {
                    device: std::mem::transmute(kernel::Device::get().device().abi()),
                },
            )?
        };

        let stage = session
            .create_reference_space(openxr::ReferenceSpaceType::STAGE, openxr::Posef::IDENTITY)?;

        let mut swapchain = session.create_swapchain(&openxr::SwapchainCreateInfo {
            create_flags: openxr::SwapchainCreateFlags::EMPTY,
            usage_flags: openxr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | openxr::SwapchainUsageFlags::SAMPLED,
            format: dxgi::DXGI_FORMAT_R8G8B8A8_UNORM.0,
            sample_count: 1,
            width: new_window_size.0,
            height: new_window_size.1,
            face_count: 1,
            array_size: VIEW_COUNT,
            mip_count: 1,
        })?;

        let swapchain_images: Vec<d3d::ID3D11Texture2D> = swapchain
            .enumerate_images()?
            .iter()
            .map(|x| unsafe { d3d::ID3D11Texture2D::from_abi(*x as *mut _) })
            .collect::<Result<Vec<d3d::ID3D11Texture2D>, _>>()?;

        let mut swapchain_desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        {
            let index = swapchain.acquire_image()?;
            swapchain.wait_image(openxr::Duration::INFINITE)?;
            unsafe {
                swapchain_images[index as usize].GetDesc(&mut swapchain_desc);
            }
            swapchain.release_image()?;
        }

        let texture_desc = d3d::D3D11_TEXTURE2D_DESC {
            Width: swapchain_desc.Width,
            Height: swapchain_desc.Height,
            MipLevels: 1,
            ArraySize: 1,
            Format: dxgi::DXGI_FORMAT_R16G16B16A16_FLOAT,
            SampleDesc: dxgi::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: d3d::D3D11_USAGE_DEFAULT,
            BindFlags: d3d::D3D11_BIND_SHADER_RESOURCE,
            CPUAccessFlags: d3d::D3D11_CPU_ACCESS_FLAG(0),
            MiscFlags: d3d::D3D11_RESOURCE_MISC_FLAG(0),
        };

        let mut buffer_images: Vec<_> = vec![];
        let mut buffer_srvs: Vec<_> = vec![];
        for _ in 0..VIEW_COUNT {
            let device = unsafe { kernel::Device::get().device() };
            let texture: d3d::ID3D11Texture2D =
                unsafe { device.CreateTexture2D(&texture_desc, std::ptr::null())? };

            let srv_desc = d3d::D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: texture_desc.Format,
                ViewDimension: d3d::D3D_SRV_DIMENSION_TEXTURE2D,
                Anonymous: d3d::D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: d3d::D3D11_TEX2D_SRV {
                        MostDetailedMip: 0,
                        MipLevels: 1,
                    },
                },
            };
            let srv = unsafe { device.CreateShaderResourceView(texture.clone(), &srv_desc)? };

            buffer_images.push(texture);
            buffer_srvs.push(srv);
        }

        unsafe {
            let window = framework::Framework::get().window_mut();
            window.set_resizing_enabled(false);
            window.set_size(new_window_size);
        }

        Ok(XR {
            instance,
            session,
            frame_waiter,
            frame_stream,
            stage,

            swapchain,
            buffer_images,
            buffer_srvs,
            swapchain_images,

            environment_blend_mode,
            frame_state: None,

            old_window_size,
            frame_size: new_window_size,

            instance_properties,
            system_properties,
            available_extensions,
        })
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        let session = &self.session;
        // TODO: do something with this
        let mut _session_running = true;
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
                            _session_running = true;
                        }
                        SessionState::STOPPING => {
                            session.end()?;
                            _session_running = false;
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

        if ct_config::xr::RUN_XR_PER_FRAME {
            self.frame_state = Some(self.frame_waiter.wait()?);
            self.frame_stream.begin()?;
        }

        Ok(())
    }

    pub fn post_update(&mut self) -> anyhow::Result<()> {
        if ct_config::xr::RUN_XR_PER_FRAME {
            let frame_state = &self
                .frame_state
                .ok_or(anyhow::Error::msg("failed to get frame state"))?;

            self.frame_stream.end(
                frame_state.predicted_display_time,
                self.environment_blend_mode,
                &[],
            )?;
        }

        Ok(())
    }

    pub fn draw_ui_framebuffers(&mut self) -> anyhow::Result<()> {
        use cimgui as ig;

        let ig::Vec2 { x: width, .. } = ig::get_window_size();
        let inverse_aspect_ratio = self.frame_size.1 as f32 / self.frame_size.0 as f32;
        let srv_width = (width * 0.5) - 32.0;

        if ig::begin_table("xivr_debug_tab_framebuffers_table", 2, None, None, None)? {
            for buffer_srv in self.buffer_srvs.iter() {
                ig::table_next_column();
                ig::image_button(
                    buffer_srv.abi(),
                    ig::Vec2::new(srv_width, srv_width * inverse_aspect_ratio),
                    None,
                    None,
                    None,
                    Some(ig::Color::new(0.0, 0.0, 0.0, 1.0)),
                    None,
                );
            }
            ig::end_table();
        }

        Ok(())
    }

    #[rustfmt::skip]
    pub fn draw_ui_properties(&mut self) -> anyhow::Result<()> {
        use cimgui as ig;

        if ig::collapsing_header("Config", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            ig::bulletf!("data.yml version: {}", crate::game::VERSION);
        }

        if ig::collapsing_header("Instance Properties", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let inst_props = &self.instance_properties;
            ig::bulletf!("Runtime name: {}", inst_props.runtime_name);
            ig::bulletf!("Runtime version: {}", inst_props.runtime_version);
        }

        if ig::collapsing_header("System Properties", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let sys_props = &self.system_properties;
            ig::bulletf!("System name: {}", sys_props.system_name);
            ig::bulletf!("Vendor ID: {}", sys_props.vendor_id);
            ig::bulletf!("System ID: {:?}", sys_props.system_id);
            ig::bulletf!("Orientation Tracking: {}", sys_props.tracking_properties.orientation_tracking);
            ig::bulletf!("Position Tracking: {}", sys_props.tracking_properties.position_tracking);
        }

        if ig::collapsing_header("Extensions", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let exts = &self.available_extensions;
            ig::bulletf!("ext_performance_settings: {}", exts.ext_performance_settings);
            ig::bulletf!("ext_debug_utils: {}", exts.ext_debug_utils);
            ig::bulletf!("ext_eye_gaze_interaction: {}", exts.ext_eye_gaze_interaction);
            ig::bulletf!("ext_hand_tracking: {}", exts.ext_hand_tracking);
            ig::bulletf!("ext_hand_joints_motion_range: {}", exts.ext_hand_joints_motion_range);
            ig::bulletf!("msft_hand_interaction: {}", exts.msft_hand_interaction);
            ig::bulletf!("msft_hand_tracking_mesh: {}", exts.msft_hand_tracking_mesh);
            ig::bulletf!("msft_controller_model: {}", exts.msft_controller_model);
        }

        if ig::collapsing_header("Frame", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            ig::bulletf!("Frame size: {}x{}", self.frame_size.0, self.frame_size.1);
            ig::bulletf!("Original window size: {}x{}", self.old_window_size.0, self.old_window_size.1);
        }

        Ok(())
    }

    pub fn copy_backbuffer_to_buffer(&mut self, index: u32) {
        unsafe {
            let dc = kernel::Device::get().device_context();
            let texture: &'static _ = render::RenderTargetManager::get().rendered_game().texture();

            dc.CopyResource(self.buffer_images[index as usize].clone(), texture.clone());
        }
    }

    pub fn _copy_buffers_to_swapchain(&mut self) -> anyhow::Result<()> {
        let mut captured = [false; VIEW_COUNT as usize];

        loop {
            let index = self.swapchain.acquire_image()? as usize;
            self.swapchain.wait_image(openxr::Duration::INFINITE)?;
            if !captured[index] {
                unsafe {
                    let device_context = kernel::Device::get().device_context_ptr();

                    (*device_context).CopyResource(
                        self.swapchain_images[index].clone(),
                        self.buffer_images[index].clone(),
                    );
                }
                captured[index] = true;
            }
            self.swapchain.release_image()?;

            if captured.iter().all(|x| *x) {
                break;
            }
        }
        Ok(())
    }
}

impl Drop for XR {
    fn drop(&mut self) {
        if ct_config::xr::CHANGE_WINDOW_SIZE {
            unsafe {
                let window: &mut _ = framework::Framework::get().window_mut();

                window.set_resizing_enabled(true);
                window.set_size(self.old_window_size);
            }
        }
    }
}
