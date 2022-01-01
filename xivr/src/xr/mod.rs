mod debug_state;
mod framebuffer;
mod framebuffer_blitter;
mod swapchain;

use crate::ct_config;
use crate::game::graphics::kernel;
use crate::game::system::framework;
use crate::singleton;

pub use crate::ct_config::xr::VIEW_COUNT;
const VIEW_TYPE: openxr::ViewConfigurationType = openxr::ViewConfigurationType::PRIMARY_STEREO;

struct FrameState {
    xr_frame_state: openxr::FrameState,
    views: Vec<openxr::View>,
}

#[allow(dead_code)]
pub struct XR {
    instance: openxr::Instance,
    session: openxr::Session<openxr::D3D11>,

    instance_properties: openxr::InstanceProperties,
    system_properties: openxr::SystemProperties,
    available_extensions: openxr::ExtensionSet,

    debug_state: Option<debug_state::DebugState>,

    frame_waiter: openxr::FrameWaiter,
    frame_stream: openxr::FrameStream<openxr::D3D11>,
    stage: openxr::Space,
    view_configuration_views: Vec<openxr::ViewConfigurationView>,
    session_running: bool,

    swapchain: swapchain::Swapchain,
    framebuffer: framebuffer::Framebuffer,
    framebuffer_blitter: framebuffer_blitter::FramebufferBlitter,

    environment_blend_mode: openxr::EnvironmentBlendMode,

    frame_state: Option<FrameState>,

    old_window_size: (u32, u32),
    view_size: (u32, u32),
}
singleton!(XR);

impl XR {
    pub fn new() -> anyhow::Result<XR> {
        let validate = cfg!(feature = "debug_validation");

        let entry = openxr::Entry::linked();
        let available_extensions = entry.enumerate_extensions()?;
        assert!(available_extensions.khr_d3d11_enable);

        let mut enabled_extensions = openxr::ExtensionSet::default();
        enabled_extensions.khr_d3d11_enable = true;
        enabled_extensions.ext_debug_utils = true;

        let mut layers = vec![];
        if validate {
            layers.push("XR_APILAYER_LUNARG_core_validation");
        }

        let instance = entry.create_instance(
            &openxr::ApplicationInfo {
                application_name: "XIVR",
                application_version: 0,
                engine_name: "XIVR",
                engine_version: 0,
            },
            &enabled_extensions,
            &layers,
        )?;
        let debug_state = validate
            .then(|| debug_state::DebugState::new(&entry, &instance))
            .transpose()?;

        let instance_properties = instance.properties()?;
        log!("xr", "created instance");

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

        let view_configuration_views =
            instance.enumerate_view_configuration_views(system, VIEW_TYPE)?;
        assert_eq!(view_configuration_views[0], view_configuration_views[1]);
        log!("xr", "enumerated view configuration views");

        let old_window_size = unsafe { framework::Framework::get().window().get_size() };
        let new_window_size = if ct_config::xr::CHANGE_WINDOW_SIZE {
            (
                view_configuration_views[0].recommended_image_rect_width,
                view_configuration_views[0].recommended_image_rect_height,
            )
        } else {
            old_window_size
        };
        let framebuffer_size = (new_window_size.0 * VIEW_COUNT, new_window_size.1);
        log!("xr", "window size: {:?}", new_window_size);
        log!("xr", "framebuffer size: {:?}", framebuffer_size);

        let device = unsafe { kernel::Device::get().device() };
        // I should figure out why this is necessary
        let device_ptr: *mut *mut () = unsafe { std::mem::transmute(device) };
        let (session, frame_waiter, frame_stream) = unsafe {
            instance.create_session::<openxr::D3D11>(
                system,
                &openxr::d3d::SessionCreateInfo {
                    device: std::mem::transmute(*device_ptr),
                },
            )?
        };
        log!("xr", "created session");

        let stage = session
            .create_reference_space(openxr::ReferenceSpaceType::STAGE, openxr::Posef::IDENTITY)?;

        unsafe {
            let window = framework::Framework::get().window_mut();
            window.set_resizing_enabled(false);
            window.set_size(new_window_size);
        }
        log!("xr", "resized window");

        let swapchain = swapchain::Swapchain::new(&session, framebuffer_size)?;
        let framebuffer = framebuffer::Framebuffer::new(device.clone(), framebuffer_size)?;
        let framebuffer_blitter = framebuffer_blitter::FramebufferBlitter::new(device.clone())?;

        log!("xr", "created swapchain");

        Ok(XR {
            instance,
            session,

            instance_properties,
            system_properties,
            available_extensions,

            debug_state,

            frame_waiter,
            frame_stream,
            stage,
            view_configuration_views,
            session_running: false,

            swapchain,
            framebuffer,
            framebuffer_blitter,

            environment_blend_mode,
            frame_state: None,

            old_window_size,
            view_size: new_window_size,
        })
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn post_update(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    pub fn pre_render(&mut self) -> anyhow::Result<()> {
        let session = &self.session;
        let mut event_storage = openxr::EventDataBuffer::new();

        while let Some(event) = self.instance.poll_event(&mut event_storage)? {
            use openxr::Event::*;
            use openxr::SessionState;
            match event {
                SessionStateChanged(e) => {
                    // Session state change is where we can begin and end sessions, as well as
                    // find quit messages!
                    log!("xr", "entered state {:?}", e.state());
                    match e.state() {
                        SessionState::READY => {
                            session.begin(VIEW_TYPE)?;
                            self.session_running = true;
                        }
                        SessionState::STOPPING => {
                            session.end()?;
                            self.session_running = false;
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
                    log!("xr", "lost {} events", e.lost_event_count());
                }
                _ => {}
            }
        }

        if self.session_running {
            let xr_frame_state = self.frame_waiter.wait()?;
            self.frame_stream.begin()?;
            if xr_frame_state.should_render {
                let (_, views) = self.session.locate_views(
                    VIEW_TYPE,
                    xr_frame_state.predicted_display_time,
                    &self.stage,
                )?;

                self.frame_state = Some(FrameState {
                    xr_frame_state,
                    views,
                });
            } else {
                // terminate stream submission immediately
                self.frame_stream.end(
                    xr_frame_state.predicted_display_time,
                    self.environment_blend_mode,
                    &[],
                )?;
                self.frame_state = None;
            }
        }
        Ok(())
    }

    pub fn post_render(&mut self) -> anyhow::Result<()> {
        if let Some(frame_state) = &self.frame_state {
            self.swapchain
                .copy_from_texture(self.framebuffer.texture())?;

            let swapchain_ref = self.swapchain.openxr_swapchain();
            let views = frame_state
                .views
                .iter()
                .enumerate()
                .map(|(index, view)| {
                    let rect = openxr::Rect2Di {
                        offset: openxr::Offset2Di {
                            x: (index * self.view_size.0 as usize) as _,
                            y: 0,
                        },
                        extent: openxr::Extent2Di {
                            width: self.view_size.0 as _,
                            height: self.view_size.1 as _,
                        },
                    };

                    openxr::CompositionLayerProjectionView::new()
                        .pose(view.pose)
                        .fov(view.fov)
                        .sub_image(
                            openxr::SwapchainSubImage::new()
                                .swapchain(swapchain_ref)
                                .image_rect(rect),
                        )
                })
                .collect::<Vec<_>>();

            self.frame_stream.end(
                frame_state.xr_frame_state.predicted_display_time,
                self.environment_blend_mode,
                &[&openxr::CompositionLayerProjection::new()
                    .space(&self.stage)
                    .views(&views)],
            )?;

            self.frame_state = None;
        }

        Ok(())
    }

    pub fn draw_ui_framebuffers(&mut self) -> anyhow::Result<()> {
        use cimgui as ig;

        let ig::Vec2 { x: width, .. } = ig::get_window_size();
        let inverse_aspect_ratio = self.view_size.1 as f32 / (VIEW_COUNT * self.view_size.0) as f32;
        let srv_width = width - 32.0;
        let size = ig::Vec2::new(srv_width, srv_width * inverse_aspect_ratio);
        let color = ig::Color::new(0.0, 0.0, 0.0, 1.0);

        ig::new_line();
        ig::same_line(None, Some(0.0));
        self.framebuffer.render_button(size, color)?;

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
            ig::bulletf!("View size: {}x{}", self.view_size.0, self.view_size.1);
            ig::bulletf!("Original window size: {}x{}", self.old_window_size.0, self.old_window_size.1);
        }

        Ok(())
    }

    pub fn copy_backbuffer_to_buffer(&mut self, index: u32) -> anyhow::Result<()> {
        unsafe { self.framebuffer_blitter.blit(&self.framebuffer, index) }
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
