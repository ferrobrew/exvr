use detour::static_detour;

static_detour! {
    pub static RenderManager_Render_Detour: fn(usize) -> usize;
    pub static RenderManager_RenderUI_Detour: fn(usize, u8) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        use crate::log;

        let res = unsafe { RenderManager_Render_Detour.disable() };
        if let Err(e) = res {
            log!("error while disabling render detour: {}", e.to_string())
        }
        let res = unsafe { RenderManager_RenderUI_Detour.disable() };
        if let Err(e) = res {
            log!("error while disabling renderui detour: {}", e.to_string())
        }
    }
}

pub unsafe fn install() -> Option<HookState> {
    use crate::module::GAME_MODULE;
    use std::mem;

    let rendermanager_render_addr = GAME_MODULE
        .get()?
        .scan("40 53 55 57 41 56 41 57 48 83 EC 60")
        .ok()?;

    RenderManager_Render_Detour
        .initialize(mem::transmute(rendermanager_render_addr), move |s| {
            use crate::debugger::Debugger;
            use crate::game::graphics::kernel::Context;
            use crate::hooks::graphics::kernel::immediate_context::XIVRCommandPayload;
            use crate::xr::{VIEW_COUNT, XR};

            let rc = Context::get_for_current_thread().unwrap();

            for i in 0..VIEW_COUNT {
                if let Some(debugger) = Debugger::get_mut() {
                    let mut command_stream = debugger.command_stream.lock().unwrap();
                    command_stream
                        .add_marker(&format!("RenderManager::Render pre-call ({})", i))
                        .unwrap();
                }
                RenderManager_Render_Detour.call(s);
                if let Some(debugger) = Debugger::get_mut() {
                    let mut command_stream = debugger.command_stream.lock().unwrap();
                    command_stream
                        .add_marker(&format!("RenderManager::Render post-call ({})", i))
                        .unwrap();
                }

                rc.push_back_xivr_command(
                    move |immediate_context, payload| {
                        if let XIVRCommandPayload::Integer(i) = payload {
                            if let Some(xr) = XR::get_mut() {
                                xr.copy_backbuffer_to_buffer(*i);
                                // xr.copy_buffers_to_swapchain().unwrap();
                                (*immediate_context.device_context_ptr()).ClearState();
                            }
                        }
                    },
                    XIVRCommandPayload::Integer(i),
                );
            }

            0usize
        })
        .ok()?;
    RenderManager_Render_Detour.enable().ok()?;

    let rendermanager_renderui_addr = GAME_MODULE
        .get()?
        .scan("48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC 40 44 8B 05 ? ? ? ?")
        .ok()?;

    RenderManager_RenderUI_Detour
        .initialize(mem::transmute(rendermanager_renderui_addr), move |s, a| {
            use crate::debugger::Debugger;
            if let Some(debugger) = Debugger::get_mut() {
                let mut command_stream = debugger.command_stream.lock().unwrap();
                command_stream
                    .add_marker("RenderManager::RenderUI pre-call")
                    .unwrap();
            }
            let ret = RenderManager_RenderUI_Detour.call(s, a);
            if let Some(debugger) = Debugger::get_mut() {
                let mut command_stream = debugger.command_stream.lock().unwrap();
                command_stream
                    .add_marker("RenderManager::RenderUI post-call")
                    .unwrap();
            }

            ret
        })
        .ok()?;
    RenderManager_RenderUI_Detour.enable().ok()?;

    Some(HookState {})
}
