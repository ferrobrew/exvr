use crate::module::Module;
use once_cell::unsync::OnceCell;

static mut GAME_MODULE: OnceCell<Module> = OnceCell::new();
static mut THIS_MODULE: OnceCell<Module> = OnceCell::new();

pub fn game_module() -> anyhow::Result<&'static Module> {
    unsafe {
        GAME_MODULE
            .get()
            .ok_or_else(|| anyhow::anyhow!("failed to retrieve module"))
    }
}

pub fn set_game_module(module: Module) -> anyhow::Result<()> {
    unsafe {
        Ok(GAME_MODULE
            .set(module)
            .map_err(|_| anyhow::Error::msg("failed to set module"))?)
    }
}

pub fn this_module() -> anyhow::Result<&'static Module> {
    unsafe {
        THIS_MODULE
            .get()
            .ok_or_else(|| anyhow::anyhow!("failed to retrieve module"))
    }
}

pub fn set_this_module(module: Module) -> anyhow::Result<()> {
    unsafe {
        Ok(THIS_MODULE
            .set(module)
            .map_err(|_| anyhow::Error::msg("failed to set module"))?)
    }
}

pub fn this_module_available() -> bool {
    unsafe { THIS_MODULE.get().is_some() }
}
