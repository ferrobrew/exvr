use crate::debugger::Debugger;
use crate::game::graphics::kernel::ShaderCommand;
use crate::log;
use crate::module::GAME_MODULE;

use detour::static_detour;

static_detour! {
    pub static Context_PushBackCmd_Detour: fn(usize, &'static ShaderCommand) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { Context_PushBackCmd_Detour.disable() };
        if let Err(e) = res {
            log!("error while disabling context detour: {}", e.to_string());
        }
    }
}

fn context_pushbackcmd_hook(ctx: usize, cmd: &'static ShaderCommand) -> usize {
    if let Some(debugger) = Debugger::get_mut() {
        let mut command_stream = debugger.command_stream.lock().unwrap();
        command_stream.add_command(cmd).unwrap();
    }
    Context_PushBackCmd_Detour.call(ctx, cmd)
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use std::mem;

    let module = GAME_MODULE
        .get()
        .ok_or(anyhow::Error::msg("Failed to retrieve game module"))?;
    let context_pushbackcmd: fn(usize, &'static ShaderCommand) -> usize =
        mem::transmute(module.scan("83 41 30 FF")?);

    Context_PushBackCmd_Detour.initialize(context_pushbackcmd, context_pushbackcmd_hook)?;
    Context_PushBackCmd_Detour.enable()?;

    Ok(HookState {})
}
