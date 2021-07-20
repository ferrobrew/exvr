#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm)]

mod game;
mod hooks;
#[macro_use]
mod log;
mod debugger;
mod module;
mod xr;
#[macro_use]
mod util;

use hooks::HookState;
use log::Logger;
use module::{Module, GAME_MODULE};

use anyhow::{Error, Result};
use bindings::Windows::Win32::Foundation::HINSTANCE;
use cimgui as ig;
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
    imgui_context: *mut ig::Context,
    imgui_allocator_alloc: ig::MemAllocFunc,
    imgui_allocator_free: ig::MemFreeFunc,
    imgui_allocator_user_data: *mut c_void,
}

impl Core {
    pub fn new(parameters: *const LoadParameters) -> Result<Core> {
        let parameters: &LoadParameters = unsafe { &*parameters };
        let mut patcher = hooks::Patcher::new();

        log!("loaded {}", game::VERSION);
        let mut modules = Module::get_all();
        let ffxiv_module = modules
            .iter_mut()
            .find(|x| x.filename().as_deref() == Some("ffxiv_dx11.exe"))
            .ok_or(Error::msg("failed to find ff14 module"))?;
        ffxiv_module.backup_image();

        unsafe {
            GAME_MODULE
                .set(ffxiv_module.clone())
                .map_err(|_| Error::msg("failed to set module"))?
        };

        debugger::Debugger::create()?;

        let hook_state =
            HookState::new(&mut patcher).ok_or(Error::msg("failed to install hooks"))?;

        xr::XR::create()?;

        unsafe {
            ig::set_current_context(parameters.imgui_context);
            ig::set_allocator_functions(
                parameters.imgui_allocator_alloc,
                parameters.imgui_allocator_free,
                parameters.imgui_allocator_user_data,
            );
        }

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
        xr::XR::destroy();
        debugger::Debugger::destroy();
    }
}

unsafe fn xivr_load_impl(parameters: *const LoadParameters) -> Result<()> {
    Logger::create((*parameters).logger)?;

    std::panic::set_hook(Box::new(|info| {
        match (info.payload().downcast_ref::<&str>(), info.location()) {
            (Some(msg), Some(loc)) => log!("Panic! {:?} at {}:{}", msg, loc.file(), loc.line()),
            (Some(msg), None) => log!("Panic! {:?}", msg),
            (None, Some(loc)) => log!("Panic! at {}:{}", loc.file(), loc.line()),
            (None, None) => log!("Panic! something at somewhere"),
        };

        log!("{:?}", backtrace::Backtrace::new_unresolved());
    }));

    let r = std::panic::catch_unwind(|| {
        CORE.set(Core::new(parameters)?)
            .map_err(|_| Error::msg("failed to set core"))
    });
    match r {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(err),
        Err(msg) => Err(Error::msg(
            msg.downcast_ref::<&str>()
                .map(|x| *x)
                .unwrap_or("Failed initialisation"),
        )),
    }
}

#[no_mangle]
pub unsafe extern "system" fn xivr_load(parameters: *const LoadParameters) -> bool {
    let result = xivr_load_impl(parameters);
    match result {
        Ok(_) => true,
        Err(e) => {
            log!("failed to initialize. {:?} {:?}", e, e.backtrace());
            false
        }
    }
}

#[no_mangle]
pub unsafe extern "system" fn xivr_unload() {
    let _ = CORE.take();
    Logger::destroy();
}

#[no_mangle]
pub unsafe extern "system" fn xivr_draw_ui() {
    if let Some(debugger) = debugger::Debugger::get_mut() {
        debugger.draw_ui().unwrap();
    }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(_module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    true
}
