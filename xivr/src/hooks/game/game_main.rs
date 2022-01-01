use crate::game::offsets;
use crate::log;
use crate::util;

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
            log!(
                "error",
                "error while disabling game detour: {}",
                e.to_string()
            )
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    let module = util::game_module_mut()?;
    let gamemain_update: fn(usize) -> usize = mem::transmute(
        module.rel_to_abs_addr(offsets::classes::game::GameMain::funcs::Update),
    );

    GameMain_Update_Detour.initialize(gamemain_update, |s| {
        util::handle_error_in_block(|| {
            use crate::debugger::Debugger;
            use crate::xr::XR;

            if let Some(debugger) = Debugger::get_mut() {
                debugger.pre_update()?;
            }

            if let Some(xr) = XR::get_mut() {
                xr.pre_update()?;
            }

            let ret = GameMain_Update_Detour.call(s);

            if let Some(xr) = XR::get_mut() {
                xr.post_update()?;
            }

            if crate::tier2_loadable() {
                let load_result = crate::load_tier2();
                if let Err(err) = load_result {
                    crate::load_fail(err.to_string());
                }
            }

            // yolo
            if cfg!(not(feature = "dalamud")) {
                use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_F7};

                if (GetAsyncKeyState(VK_F7.0.into()) & 0x01) != 0 {
                    crate::xivr_unload();
                }
            }

            Ok(ret)
        })
    })?;
    GameMain_Update_Detour.enable()?;

    Ok(HookState {})
}
