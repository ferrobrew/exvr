use crate::debugger::Debugger;
use crate::game::graphics::kernel::ShaderCommand;
use crate::hooks;
use crate::log;
use crate::module::GAME_MODULE;

use detour::static_detour;

static_detour! {
    pub static ImmediateContext_PushBackCmd_Detour: fn(usize, &'static ShaderCommand) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { ImmediateContext_PushBackCmd_Detour.disable() };
        if let Err(e) = res {
            log!(
                "error while disabling immediate context detour: {}",
                e.to_string()
            );
        }
    }
}

fn immediatecontext_pushbackcmd_hook(ctx: usize, cmd: &'static ShaderCommand) -> usize {
    if let Some(debugger) = Debugger::get_mut() {
        debugger.command_stream.add_command(cmd).unwrap();
    }
    ImmediateContext_PushBackCmd_Detour.call(ctx, cmd)
}

pub unsafe fn install(_patcher: &mut hooks::Patcher) -> Option<HookState> {
    use std::mem;

    let module = GAME_MODULE.get()?;
    let immediatecontext_pushbackcmd: fn(usize, &'static ShaderCommand) -> usize =
        mem::transmute(module.scan("83 41 30 FF").ok()?);

    ImmediateContext_PushBackCmd_Detour
        .initialize(
            immediatecontext_pushbackcmd,
            immediatecontext_pushbackcmd_hook,
        )
        .ok()?;

    ImmediateContext_PushBackCmd_Detour.enable().ok()?;

    Some(HookState {})
}
