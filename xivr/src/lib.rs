#![feature(global_asm)]
#![feature(naked_functions)]
#![feature(asm)]
#![feature(core_intrinsics)]

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
use module::Module;

use std::os::raw::c_void;

use once_cell::sync::Lazy;
use std::sync::Mutex;

use anyhow::{Error, Result};
use cimgui as ig;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::Console::{AllocConsole, FreeConsole};

#[derive(PartialEq)]
enum LoadState {
    Init,
    Tier1Loaded,
    Tier2Loaded,
    Failure(String),
}
static LOAD_STATE: Lazy<Mutex<LoadState>> = Lazy::new(|| Mutex::new(LoadState::Init));

#[repr(C, packed)]
pub struct LoadParameters {
    logger: log::LogType,
    imgui_context: *mut ig::Context,
    imgui_allocator_alloc: ig::MemAllocFunc,
    imgui_allocator_free: ig::MemFreeFunc,
    imgui_allocator_user_data: *mut c_void,
}

unsafe fn patch_symbol_search_path() -> Result<()> {
    use windows::Win32::Foundation::PWSTR;
    use windows::Win32::System::Diagnostics::Debug::{SymGetSearchPathW, SymSetSearchPathW};
    use windows::Win32::System::Threading::GetCurrentProcess;

    let current_process = GetCurrentProcess();
    let directory = util::this_module_directory()?
        .to_str()
        .ok_or_else(|| anyhow::anyhow!("failed to get path string"))?;

    // This is very silly. We would like to mutate the search path of the process
    // to include where our DLLs came from, but we can't do that without being sure
    // that dbghelp has been loaded and SymInitialize has been called. The backtrace
    // crate will do this for us, but only when a backtrace occurs. So... let's
    // call backtrace to force initialisation!
    backtrace::Backtrace::new_unresolved();

    let path = {
        let mut buf = vec![0u16; 1024];
        if !SymGetSearchPathW(current_process, PWSTR(buf.as_mut_ptr()), buf.len() as u32).as_bool()
        {
            return Err(anyhow::anyhow!("failed to get search path"));
        }

        let len = buf
            .iter()
            .position(|c| *c == 0)
            .ok_or_else(|| anyhow::anyhow!("failed to retrieve length"))?;

        String::from_utf16(&buf[..len])?
    };

    let new_path = if path.contains(directory) {
        path
    } else {
        format!("{};{}", directory, &path)
    };

    let mut new_buf: Vec<u16> = new_path.encode_utf16().collect();
    new_buf.push(0);
    if !SymSetSearchPathW(current_process, PWSTR(new_buf.as_mut_ptr())).as_bool() {
        return Err(anyhow::anyhow!("failed to set search path"));
    }

    Ok(())
}

unsafe fn load_tier1(parameters: Option<&LoadParameters>) -> Result<()> {
    log!("tier1", "start");
    let mut modules = Module::get_all();
    let ffxiv_module = modules
        .iter_mut()
        .find(|x| x.filename().as_deref() == Some("ffxiv_dx11.exe"))
        .ok_or_else(|| Error::msg("failed to find ff14 module"))?;
    ffxiv_module.backup_image();
    ffxiv_module.load_cache()?;

    util::set_game_module(ffxiv_module.clone())?;
    log!("tier1", "located module");

    patch_symbol_search_path()?;
    log!("tier1", "patched symbol search path");

    hooks::Patcher::create()?;
    debugger::Debugger::create()?;
    HookState::create()?;
    log!("tier1", "installed hooks");

    util::game_module_mut()?.save_cache()?;

    if let Some(parameters) = parameters {
        ig::set_current_context(parameters.imgui_context);
        ig::set_allocator_functions(
            parameters.imgui_allocator_alloc,
            parameters.imgui_allocator_free,
            parameters.imgui_allocator_user_data,
        );
        log!("tier1", "initialised imgui");
    }
    *LOAD_STATE.lock().unwrap() = LoadState::Tier1Loaded;
    log!("tier1", "complete");

    Ok(())
}

unsafe fn load_tier2() -> Result<()> {
    log!("tier2", "start");
    xr::XR::create()?;
    *LOAD_STATE.lock().unwrap() = LoadState::Tier2Loaded;
    log!("tier2", "complete");
    Ok(())
}

fn tier2_loadable() -> bool {
    *LOAD_STATE.lock().unwrap() == LoadState::Tier1Loaded
}

fn load_fail(msg: String) {
    log!("load", "failed: {}", msg);
    *LOAD_STATE.lock().unwrap() = LoadState::Failure(msg);
}

unsafe fn xivr_load_impl(parameters: *const LoadParameters) -> Result<()> {
    if cfg!(not(feature = "dalamud")) {
        use c_str_macro::c_str;
        use libc::{fdopen, freopen};

        AllocConsole();

        let stdout = fdopen(1, c_str!("w").as_ptr());
        let stderr = fdopen(2, c_str!("w").as_ptr());
        freopen(c_str!("CONOUT$").as_ptr(), c_str!("w").as_ptr(), stdout);
        freopen(c_str!("CONOUT$").as_ptr(), c_str!("w").as_ptr(), stderr);
    }

    let parameters = parameters.as_ref();
    Logger::create(parameters.map(|x| x.logger))?;

    std::panic::set_hook(Box::new(|info| {
        match (info.payload().downcast_ref::<&str>(), info.location()) {
            (Some(msg), Some(loc)) => {
                log!("panic", "Panic! {:?} at {}:{}", msg, loc.file(), loc.line())
            }
            (Some(msg), None) => log!("panic", "Panic! {:?}", msg),
            (None, Some(loc)) => log!("panic", "Panic! at {}:{}", loc.file(), loc.line()),
            (None, None) => log!("panic", "Panic! something at somewhere"),
        };

        log!("panic", "{:?}", backtrace::Backtrace::new_unresolved());
    }));

    let r = std::panic::catch_unwind(|| load_tier1(parameters));
    match r {
        Ok(Ok(())) => Ok(()),
        Ok(Err(err)) => Err(err),
        Err(msg) => Err(Error::msg(
            msg.downcast_ref::<&str>()
                .copied()
                .unwrap_or("Failed initialisation"),
        )),
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn xivr_load(parameters: *const LoadParameters) -> bool {
    let result = xivr_load_impl(parameters);
    util::handle_error(result).is_some()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "system" fn xivr_unload() {
    log!("xivr", "unloading!");
    HookState::destroy();
    hooks::Patcher::destroy();

    std::thread::spawn(|| {
        use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
        std::thread::sleep(std::time::Duration::from_millis(100)); // bodge

        xr::XR::destroy();
        debugger::Debugger::destroy();

        Logger::destroy();

        if cfg!(not(feature = "dalamud")) {
            FreeConsole();
        }

        FreeLibraryAndExitThread(util::this_module().unwrap().handle(), 0);
    });
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
#[cfg(feature = "dalamud")]
pub unsafe extern "system" fn xivr_draw_ui() {
    util::handle_error_in_block(|| {
        if let Some(debugger) = debugger::Debugger::get_mut() {
            debugger.draw_ui()?;
        }
        Ok(())
    });
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
#[cfg(feature = "dalamud")]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, _reason: u32, _: *mut c_void) -> bool {
    if !util::this_module_available() {
        util::set_this_module(Module::from_handle(module)).unwrap();
    }
    true
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(clippy::missing_safety_doc)]
#[cfg(not(feature = "dalamud"))]
pub unsafe extern "system" fn DllMain(module: HINSTANCE, reason: u32, _: *mut c_void) -> bool {
    use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;
    if !util::this_module_available() {
        util::set_this_module(Module::from_handle(module)).unwrap();
    }

    match reason {
        DLL_PROCESS_ATTACH => {
            std::thread::spawn(|| {
                xivr_load(std::ptr::null());
            });
        }
        _ => {}
    }
    true
}
