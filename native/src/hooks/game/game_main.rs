use crate::game::offsets;
use crate::log;
use crate::module::GAME_MODULE;

use detour::static_detour;
use std::mem;

static_detour! {
    pub static GameMain_Update_Detour: fn(usize) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { GameMain_Update_Detour.disable() };
        if let Err(e) = res {
            log!("error while disabling game detour: {}", e.to_string())
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    let module = GAME_MODULE
        .get()
        .ok_or(anyhow::Error::msg("Failed to retrieve game module"))?;
    let gamemain_update: fn(usize) -> usize =
        mem::transmute(module.rel_to_abs_addr(offsets::functions::Game_GameMain_Update as isize));

    GameMain_Update_Detour.initialize(mem::transmute(gamemain_update), |s| {
        use crate::debugger::Debugger;
        use crate::xr::XR;
        if let Some(debugger) = Debugger::get_mut() {
            debugger.pre_update().unwrap();
        }

        if let Some(xr) = XR::get_mut() {
            xr.pre_update().unwrap();
        }

        let ret = GameMain_Update_Detour.call(s);

        if let Some(xr) = XR::get_mut() {
            xr.post_update().unwrap();
        }
        ret
    })?;
    GameMain_Update_Detour.enable()?;

    Ok(HookState {})
}
