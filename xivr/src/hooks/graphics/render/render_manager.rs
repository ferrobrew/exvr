use crate::ct_config::*;
use crate::util;

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
            log!("error", "error while disabling render detour: {}", e.to_string())
        }
        let res = unsafe { RenderManager_RenderUI_Detour.disable() };
        if let Err(e) = res {
            log!("error", "error while disabling renderui detour: {}", e.to_string())
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use crate::module::GAME_MODULE;
    use std::mem;

    let module = GAME_MODULE
        .get()
        .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?;

    let rendermanager_render_addr = module.scan("40 53 55 57 41 56 41 57 48 83 EC 60")?;

    RenderManager_Render_Detour.initialize(
        mem::transmute(rendermanager_render_addr),
        move |s| {
            util::handle_error_in_block(|| {
                if rendering::DISABLE_GAME {
                    return Ok(0usize);
                }

                use crate::debugger::Debugger;
                if let Some(debugger) = Debugger::get_mut() {
                    if let Ok(mut command_stream) = debugger.command_stream.lock() {
                        command_stream.add_marker("RenderManager::Render pre-call".to_owned())?;
                    }
                }
                RenderManager_Render_Detour.call(s);
                if let Some(debugger) = Debugger::get_mut() {
                    if let Ok(mut command_stream) = debugger.command_stream.lock() {
                        command_stream.add_marker("RenderManager::Render post-call".to_owned())?;
                    }
                }

                Ok(0usize)
            })
        },
    )?;
    RenderManager_Render_Detour.enable()?;

    let rendermanager_renderui_addr = module
        .scan("48 89 5C 24 ? 48 89 6C 24 ? 56 57 41 54 41 56 41 57 48 83 EC 40 44 8B 05 ? ? ? ?")?;

    RenderManager_RenderUI_Detour.initialize(
        mem::transmute(rendermanager_renderui_addr),
        move |s, a| {
            util::handle_error_in_block(|| {
                if rendering::DISABLE_UI {
                    return Ok(0usize);
                }

                use crate::debugger::Debugger;
                if let Some(debugger) = Debugger::get_mut() {
                    if let Ok(mut command_stream) = debugger.command_stream.lock() {
                        command_stream.add_marker("RenderManager::RenderUI pre-call".to_owned())?;
                    }
                }
                let ret = RenderManager_RenderUI_Detour.call(s, a);
                if let Some(debugger) = Debugger::get_mut() {
                    if let Ok(mut command_stream) = debugger.command_stream.lock() {
                        command_stream.add_marker("RenderManager::RenderUI post-call".to_owned())?;
                    }
                }

                Ok(ret)
            })
        },
    )?;
    RenderManager_RenderUI_Detour.enable()?;

    Ok(HookState {})
}
