#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm)]

mod game;
mod hooks;
#[macro_use]
mod log;
mod module;
mod xr;

use hooks::HookState;
use log::Logger;
use module::{Module, GAME_MODULE};

use anyhow::{Error, Result};
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
    pub fn new(_parameters: LoadParameters) -> Result<Core> {
        let mut patcher = hooks::Patcher::new();

        log!("loaded {}", game::VERSION);
        let modules = Module::get_all();
        let ffxiv_module = modules
            .iter()
            .find(|x| x.filename().as_deref() == Some("ffxiv_dx11.exe"))
            .ok_or(Error::msg("failed to find ff14 module"))?;

        unsafe {
            GAME_MODULE
                .set(ffxiv_module.clone())
                .map_err(|_| Error::msg("failed to set module"))?
        };
        let hook_state =
            HookState::new(&mut patcher).ok_or(Error::msg("failed to install hooks"))?;

        unsafe {
            xr::XR_INSTANCE
                .set(xr::XR::new()?)
                .map_err(|_| Error::msg("failed to set XR"))?
        };

        log!("good to go!");

        Ok(Core {
            _patcher: patcher,
            _hook_state: hook_state,
        })
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        log!("unloading!");
        let _ = unsafe { xr::XR_INSTANCE.take().unwrap() };
    }
}

unsafe fn xivr_load_impl(parameters: LoadParameters) -> Result<()> {
    Logger::initialize_instance(parameters.logger);

    std::panic::set_hook(Box::new(|info| {
        match (info.payload().downcast_ref::<&str>(), info.location()) {
            (Some(msg), Some(loc)) => log!("Panic! {:?} at {}:{}", msg, loc.file(), loc.line()),
            (Some(msg), None) => log!("Panic! {:?}", msg),
            (None, Some(loc)) => log!("Panic! at {}:{}", loc.file(), loc.line()),
            (None, None) => log!("Panic! something at somewhere")
        };
    }));

    let r = std::panic::catch_unwind(|| {
        CORE.set(Core::new(parameters)?)
            .map_err(|_| Error::msg("failed to set core"))
    });
    match r {
        Ok(Ok(())) => Ok(()),
        Ok(Err(e)) => {
            Err(Error::msg("Failed initialisation"))
        }
        Err(e) => {
            Err(Error::msg("Failed initialisation"))
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn xivr_load(parameters: LoadParameters) -> bool {
    let result = xivr_load_impl(parameters);
    match result {
        Ok(_) => true,
        Err(e) => {
            log!("failed to initialize, {}", e.to_string());
            false
        }
    }
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
