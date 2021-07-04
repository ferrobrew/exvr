#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm)]

mod game;
mod hooks;
#[macro_use]
mod log;
mod module;

use hooks::HookState;
use log::Logger;
use module::{Module, GAME_MODULE};

use bindings::Windows::Win32::Foundation::HINSTANCE;
use once_cell::unsync::OnceCell;
use std::os::raw::c_void;

static mut CORE: OnceCell<Core> = OnceCell::new();

struct Core {
    _patcher: hooks::Patcher,
    _hook_state: HookState,
}

#[repr(C, packed)]
pub struct LoadParameters {
    logger: log::LogType,
}

impl Core {
    pub fn new(_parameters: LoadParameters) -> Option<Core> {
        let mut patcher = hooks::Patcher::new();

        log!("loaded {}", game::VERSION);
        let modules = Module::get_all();
        let ffxiv_module = modules
            .iter()
            .find(|x| x.filename().as_deref() == Some("ffxiv_dx11.exe"))?;

        unsafe { GAME_MODULE.set(ffxiv_module.clone()).ok()? };
        let hook_state = HookState::new(&mut patcher)?;

        Some(Core {
            _patcher: patcher,
            _hook_state: hook_state,
        })
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        log!("unloading!");
    }
}

unsafe fn xivr_load_impl(parameters: LoadParameters) -> Option<()> {
    Logger::initialize_instance(parameters.logger);
    CORE.set(Core::new(parameters)?).ok()
}

#[no_mangle]
pub unsafe extern "system" fn xivr_load(parameters: LoadParameters) -> bool {
    xivr_load_impl(parameters).is_some()
}

#[no_mangle]
pub unsafe extern "system" fn xivr_unload() {
    let _ = CORE.take();
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    true
}
