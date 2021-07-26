use crate::game::graphics::kernel::ImmediateContext;
use crate::hooks;
use crate::log;
use crate::module::GAME_MODULE;
use detour::RawDetour;

#[no_mangle]
static mut PROCESS_EVENTS_DEFAULT_CASE: *mut u8 = std::ptr::null_mut();
const PROCESS_EVENTS_TABLE_LENGTH: u32 = 18;
const SHADER_COMMAND_HIJACKED_TYPE: usize = 9;

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
                "error while disabling immediate context detour: {}",
                e.to_string()
            )
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    let module = GAME_MODULE
        .get()
        .ok_or(anyhow::Error::msg("Failed to retrieve game module"))?;

    let process_events =
        module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 4B 30 FF 15 ? ? ? ?")?;

    let padding = module.scan_after_ptr(process_events, &"CC ".repeat(10))?;
    let padding_detour =
        RawDetour::new(padding as *const (), process_events_trampoline as *const ())?;
    padding_detour.enable()?;

    let jump_table_slice = {
        let jump_table = padding.offset(-(PROCESS_EVENTS_TABLE_LENGTH as isize) * 4);
        std::slice::from_raw_parts_mut(jump_table as *mut u32, PROCESS_EVENTS_TABLE_LENGTH as usize)
    };

    let default_offset = jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE];
    PROCESS_EVENTS_DEFAULT_CASE = module.rel_to_abs_addr(default_offset as isize);

    let padding_rel = module.abs_to_rel_addr(padding) as i32;
    hooks::Patcher::get_mut()
        .ok_or(anyhow::Error::msg("Failed to retrieve patcher"))?
        .patch(
            (&mut jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE]) as *mut _ as *mut u8,
            &padding_rel.to_ne_bytes(),
        );

    Ok(HookState(padding_detour))
}

global_asm!(
    ".global process_events_trampoline",
    "process_events_trampoline:",
    "   MOV rdx, r10",
    "   MOV rcx, rbx",
    "   MOVABS rax, PROCESS_EVENTS_DEFAULT_CASE",
    "   PUSH rax",
    "   JMP process_events_jump_table_9",
);
extern "C" {
    fn process_events_trampoline();
}

#[no_mangle]
unsafe extern "C" fn process_events_jump_table_9(
    context: &'static ImmediateContext,
    cmd: *const XIVRCommand,
) {
    ((*cmd).callback)(context, &(*cmd).payload);
}
