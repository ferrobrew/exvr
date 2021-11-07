// E8 ? ? ? ? C6 83 ? ? ? ? ? 48 8B 4B 70

use crate::debugger::Debugger;
use crate::game::graphics::kernel::ShaderCommand;
use crate::module::GAME_MODULE;
use crate::{log, util};

use detour::static_detour;

static_detour! {
    pub static Swapchain_Present_Detour: fn(usize);
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { Swapchain_Present_Detour.disable() };
        if let Err(e) = res {
            log!("error", "error while disabling swapchain detour: {}", e.to_string());
        }
    }
}

fn swapchain_present_hook(swapchain: usize) {
    if !crate::ct_config::rendering::DISABLE_SWAPCHAIN_PRESENT {
        Swapchain_Present_Detour.call(swapchain);
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use std::mem;

    let module = GAME_MODULE
        .get()
        .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?;
    let swapchain_present: fn(usize) =
        mem::transmute(module.scan_for_relative_callsite("E8 ? ? ? ? C6 83 ? ? ? ? ? 48 8B 4B 70")?);

    Swapchain_Present_Detour.initialize(swapchain_present, swapchain_present_hook)?;
    Swapchain_Present_Detour.enable()?;

    Ok(HookState {})
}
