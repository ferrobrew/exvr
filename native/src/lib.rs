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
mod ct_config;

use hooks::HookState;
use log::Logger;
use module::{Module, GAME_MODULE};

use anyhow::{Error, Result};
use bindings::Windows::Win32::Foundation::HINSTANCE;
use cimgui as ig;
use std::os::raw::c_void;

#[repr(C, packed)]
pub struct LoadParameters {
    logger: log::LogType,
    imgui_context: *mut ig::Context,
    imgui_allocator_alloc: ig::MemAllocFunc,
    imgui_allocator_free: ig::MemFreeFunc,
    imgui_allocator_user_data: *mut c_void,
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
        let parameters: &LoadParameters = unsafe { &*parameters };
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

        hooks::Patcher::create()?;
        debugger::Debugger::create()?;
        HookState::create()?;
        xr::XR::create()?;

        unsafe {
            ig::set_current_context(parameters.imgui_context);
            ig::set_allocator_functions(
                parameters.imgui_allocator_alloc,
                parameters.imgui_allocator_free,
                parameters.imgui_allocator_user_data,
            );
        }

        log!("loaded {}", game::VERSION);

        Ok(())
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
    log!("unloading!");
    xr::XR::destroy();
    HookState::destroy();
    debugger::Debugger::destroy();
    hooks::Patcher::destroy();

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
