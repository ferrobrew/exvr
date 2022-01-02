use crate::ct_config::rendering::SHADER_COMMAND_HIJACKED_TYPE;
use crate::game::graphics::kernel::{ImmediateContext, ShaderCommand};
use crate::{hooks, log, util};
use detour::{static_detour, RawDetour};

#[no_mangle]
static mut PROCESS_COMMANDS_DEFAULT_CASE: *mut u8 = std::ptr::null_mut();
const PROCESS_COMMANDS_TABLE_LENGTH: u32 = 18;

static_detour! {
    pub static ImmediateContext_ProcessCommands_Detour: fn(&'static ImmediateContext, u64, u32) -> u64;
}

#[allow(dead_code)]
pub enum XIVRCommandPayload {
    None,
    Integer(u32),
    Address(*const u8),
}

#[repr(C)]
pub struct XIVRCommand {
    cmd_type: u32,
    callback: fn(context: &'static ImmediateContext, &XIVRCommandPayload) -> (),
    payload: XIVRCommandPayload,
}

#[allow(dead_code)]
impl XIVRCommand {
    pub fn new(
        callback: fn(context: &'static ImmediateContext, &XIVRCommandPayload) -> (),
        payload: XIVRCommandPayload,
    ) -> XIVRCommand {
        XIVRCommand {
            cmd_type: SHADER_COMMAND_HIJACKED_TYPE as u32,
            callback,
            payload,
        }
    }
}

pub struct HookState(detour::RawDetour);
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { self.0.disable() };
        if let Err(e) = res {
            log!(
                "error",
                "error while disabling immediate context detour: {}",
                e.to_string()
            )
        }
        let res = unsafe { ImmediateContext_ProcessCommands_Detour.disable() };
        if let Err(e) = res {
            log!(
                "error",
                "error while disabling process commands detour: {}",
                e.to_string()
            )
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    let module = util::game_module_mut()?;
    let process_commands =
        module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 4B 30 FF 15 ? ? ? ?")?;

    ImmediateContext_ProcessCommands_Detour.initialize(
        std::mem::transmute(process_commands),
        |ic, a2, command_count| {
            util::handle_error_in_block(|| {
                use crate::debugger::Debugger;
                use crate::xr::XR;

                #[repr(C)]
                struct StreamCommand {
                    sort_key: u64,
                    cmd: *mut ShaderCommand,
                }

                let p = a2 as *mut StreamCommand;

                if let Some(debugger) = Debugger::get_mut() {
                    if let Ok(mut command_stream) = debugger.command_stream.lock() {
                        if command_stream.is_capturing() {
                            for i in 0..command_count {
                                let stream_cmd: &StreamCommand = &*p.add(i as usize);
                                let cmd: &ShaderCommand = &*stream_cmd.cmd;

                                command_stream.add_processed_command(cmd)?;
                            }
                        }
                    }
                }

                if let Some(xr) = XR::get_mut() {
                    xr.pre_render()?;
                    for i in 0..2 {
                        ImmediateContext_ProcessCommands_Detour.call(ic, a2, command_count);
                        xr.copy_backbuffer_to_buffer(i)?;
                    }
                    xr.post_render()?;
                } else {
                    ImmediateContext_ProcessCommands_Detour.call(ic, a2, command_count);
                }
                Ok(0u64)
            })
        },
    )?;
    ImmediateContext_ProcessCommands_Detour.enable()?;

    let padding = module.scan_after_ptr(process_commands, &"CC ".repeat(10))?;
    let padding_detour = RawDetour::new(
        padding as *const (),
        process_commands_jump_trampoline as *const (),
    )?;
    padding_detour.enable()?;

    let jump_table_slice = {
        let jump_table = padding.offset(-(PROCESS_COMMANDS_TABLE_LENGTH as isize) * 4);
        std::slice::from_raw_parts_mut(
            jump_table as *mut u32,
            PROCESS_COMMANDS_TABLE_LENGTH as usize,
        )
    };

    let default_offset = jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE] as usize;
    PROCESS_COMMANDS_DEFAULT_CASE = module.rel_to_abs_addr(default_offset);

    let padding_rel = module.abs_to_rel_addr(padding) as i32;
    hooks::Patcher::get_mut()
        .ok_or_else(|| anyhow::Error::msg("Failed to retrieve patcher"))?
        .patch(
            (&mut jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE]) as *mut _ as *mut u8,
            &padding_rel.to_ne_bytes(),
        );

    Ok(HookState(padding_detour))
}

extern "C" {
    fn process_commands_jump_trampoline();
}
global_asm!(
    ".global process_commands_jump_trampoline",
    "process_commands_jump_trampoline:",
    "   MOV rdx, r10",
    "   MOV rcx, rbx",
    "   MOVABS rax, PROCESS_COMMANDS_DEFAULT_CASE",
    "   PUSH rax",
    "   JMP process_commands_jump_table_9",
);

#[no_mangle]
unsafe extern "C" fn process_commands_jump_table_9(
    context: &'static ImmediateContext,
    cmd: *const XIVRCommand,
) {
    ((*cmd).callback)(context, &(*cmd).payload);
}
