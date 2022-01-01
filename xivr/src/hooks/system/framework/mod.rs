use crate::log;
use crate::util;

use detour::static_detour;
use std::mem;

static_detour! {
    pub static Framework_Tick_Detour: fn(usize) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { Framework_Tick_Detour.disable() };
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
    use crate::game::offsets::classes::system::framework::Framework;
    let module = util::game_module_mut()?;
    let framework_vtbl = module.rel_to_abs_addr(Framework::VTBLS[0] as usize) as *mut usize;
    let framework_tick_ptr = framework_vtbl.add(Framework::vfuncs::Tick).read();
    let framework_tick: fn(usize) -> usize = mem::transmute(framework_tick_ptr);

    Framework_Tick_Detour.initialize(framework_tick, |f| {
        util::handle_error_in_block(|| {
            use crate::debugger::Debugger;
            use crate::xr::XR;

            if let Some(debugger) = Debugger::get_mut() {
                debugger.pre_update()?;
            }

            if let Some(xr) = XR::get_mut() {
                xr.pre_update()?;
            }

            let ret = Framework_Tick_Detour.call(f);

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
    Framework_Tick_Detour.enable()?;

    Ok(HookState {})
}
