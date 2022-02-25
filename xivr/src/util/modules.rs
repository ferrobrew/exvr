use crate::module::Module;
use anyhow::anyhow;
use once_cell::unsync::OnceCell;

static mut GAME_MODULE: OnceCell<Module> = OnceCell::new();
static mut THIS_MODULE: OnceCell<Module> = OnceCell::new();

pub fn game_module_mut() -> anyhow::Result<&'static mut Module> {
    unsafe {
        GAME_MODULE
            .get_mut()
            .ok_or_else(|| anyhow!("failed to retrieve module"))
    }
}

pub fn set_game_module(module: Module) -> anyhow::Result<()> {
    unsafe {
        GAME_MODULE
            .set(module)
            .map_err(|_| anyhow!("failed to set module"))
    }
}

pub fn this_module() -> anyhow::Result<&'static Module> {
    unsafe {
        THIS_MODULE
            .get()
            .ok_or_else(|| anyhow!("failed to retrieve module"))
    }
}

pub fn this_module_directory() -> anyhow::Result<&'static std::path::Path> {
    this_module()?
        .directory()
        .ok_or_else(|| anyhow!("failed to get directory"))
}

pub fn set_this_module(module: Module) -> anyhow::Result<()> {
    unsafe {
        THIS_MODULE
            .set(module)
            .map_err(|_| anyhow!("failed to set module"))
    }
}

pub fn this_module_available() -> bool {
    unsafe { THIS_MODULE.get().is_some() }
}
