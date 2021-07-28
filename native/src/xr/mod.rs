use crate::ct_config;
use crate::game::graphics::kernel;
use crate::game::graphics::render;
use crate::game::system::framework;
use crate::singleton;

use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
use bindings::Windows::Win32::Graphics::Dxgi as dxgi;
use windows::Abi;

use cimgui as ig;
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

    tracked_rt_index: u32,
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

        let instance_props = instance.properties()?;
        log!(
            "loaded OpenXR runtime: {} {}",
            instance_props.runtime_name,
            instance_props.runtime_version
        );

        log!("extensions of interest supported:");
        log!(
            "  ext_performance_settings: {}",
            available_extensions.ext_performance_settings
        );
        log!(
            "  ext_debug_utils: {}",
            available_extensions.ext_debug_utils
        );
        log!(
            "  ext_eye_gaze_interaction: {}",
            available_extensions.ext_eye_gaze_interaction
        );
        log!(
            "  ext_hand_tracking: {}",
            available_extensions.ext_hand_tracking
        );
        log!(
            "  ext_hand_joints_motion_range: {}",
            available_extensions.ext_hand_joints_motion_range
        );
        log!(
            "  msft_hand_interaction: {}",
            available_extensions.msft_hand_interaction
        );
        log!(
            "  msft_hand_tracking_mesh: {}",
            available_extensions.msft_hand_tracking_mesh
        );
        log!(
            "  msft_controller_model: {}",
            available_extensions.msft_controller_model
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

        let window: &framework::Window = framework::Framework::get().window;
        let old_window_size = window.get_size();
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
                    device: std::mem::transmute((*kernel::Device::get().device_ptr()).abi()),
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
            let device = unsafe { (*kernel::Device::get().device_ptr()).clone() };
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

        window.set_resizing_enabled(false);
        window.set_size(new_window_size);

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

            tracked_rt_index: 0,
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
                let size = ig::Vec2::new(
                    self.frame_size.0 as f32 / 8.0,
                    self.frame_size.1 as f32 / 8.0,
                );

        if ig::begin_table("xivr_debug_tab_framebuffers_table", 2, None, None, None)? {
            for buffer_srv in self.buffer_srvs.iter() {
                ig::table_next_column();
                ig::image(
                    buffer_srv.abi(),
                    size,
                    None,
                    None,
                    None,
                    Some(ig::Color::ONE),
                );
            }
            ig::end_table();
        }

        unsafe {
            let texture: &'static _ = *((*kernel::Device::get().swapchain_ptr()).back_buffer_ptr());
            ig::image(
                (*texture.shader_resource_view_ptr()).abi(),
                size,
                None,
                None,
                None,
                Some(ig::Color::ONE),
            );
        }

        Ok(())
    }

    pub fn draw_ui_render_targets(&mut self) -> anyhow::Result<()> {
        let textures = render::RenderTargetManager::get().get_render_targets();

        {
            let texture = textures[self.tracked_rt_index as usize].1;
            let srv = unsafe { &(*(*texture).shader_resource_view_ptr()) };
            let size = ig::Vec2::new(
                self.frame_size.0 as f32 / 8.0,
                self.frame_size.1 as f32 / 8.0,
            );
            ig::image(srv.abi(), size, None, None, None, None);
        }

        if ig::begin_table("xivr_debug_tab_rt_list_table", 7, None, None, None)? {
            ig::table_setup_column("Active", None, None, None)?;
            ig::table_setup_column("Offset", None, None, None)?;
            ig::table_setup_column("Address", None, None, None)?;
            ig::table_setup_column("Width", None, None, None)?;
            ig::table_setup_column("Height", None, None, None)?;
            ig::table_setup_column("Format", None, None, None)?;
            ig::table_headers_row();

            for (i, (offset, texture)) in textures.into_iter().enumerate() {
                let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
                unsafe {
                    (*(*texture).texture_ptr()).GetDesc(&mut desc);
                }

                ig::table_next_row(None, None);
                ig::table_next_column();
                {
                    let tracked = self.tracked_rt_index == (i as u32);
                    let label = format!("{}###{}", if tracked { "X" } else { " " }, i);
                    if ig::button(&label, None)? {
                        self.tracked_rt_index = i as u32;
                    }
                }
                {
                    ig::table_next_column();
                    ig::textf!("{:X}", offset);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{:X?}", texture);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{}", desc.Width);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{}", desc.Height);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{:?}", desc.Format);
                }
            }

            ig::end_table();
        }

        Ok(())
    }

    pub fn copy_backbuffer_to_buffer(&mut self, index: u32) {
        unsafe {
            let device_context = kernel::Device::get().device_context_ptr();
            let texture: &'static _ = *render::RenderTargetManager::get().rendered_game_ptr();

            (*device_context).CopyResource(
                self.buffer_images[index as usize].clone(),
                (*texture.texture_ptr()).clone(),
            );
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
            framework::Framework::get()
                .window
                .set_resizing_enabled(true);
            framework::Framework::get()
                .window
                .set_size(self.old_window_size);
        }
    }
}
