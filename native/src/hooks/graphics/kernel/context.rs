use crate::hooks;
use crate::module::Module;
use crate::log;
use detour::RawDetour;

#[no_mangle]
pub static mut PROCESS_EVENTS_DEFAULT_CASE: *mut u8 = std::ptr::null_mut();
const PROCESS_EVENTS_TABLE_LENGTH: u32 = 18;
const SHADER_COMMAND_HIJACKED_TYPE: usize = 9;

#[repr(C)]
pub struct ShaderCommandXIVR {
    cmd_type: u32,
    callback: fn() -> (),
}

impl ShaderCommandXIVR {
    pub fn new(callback: fn() -> ()) -> ShaderCommandXIVR {
        ShaderCommandXIVR {
            cmd_type: SHADER_COMMAND_HIJACKED_TYPE as u32,
            callback,
        }
    }
}

pub struct HookState(detour::RawDetour);
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { self.0.disable() };
        if let Err(e) = res {
            log!("error while disabling context detour: {}", e.to_string())
        }
    }
}

pub unsafe fn patch_process_events(
    module: &Module,
    patcher: &mut hooks::Patcher,
) -> Option<HookState> {
    let process_events =
        module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 4B 30 FF 15 ? ? ? ?")?;

    let padding = module.scan_after_ptr(process_events, &"CC ".repeat(10))?;
    let padding_detour =
        RawDetour::new(padding as *const (), process_events_trampoline as *const ()).ok()?;
    padding_detour.enable().ok()?;

    let jump_table_slice = {
        let jump_table = padding.offset(-(PROCESS_EVENTS_TABLE_LENGTH as isize) * 4);
        std::slice::from_raw_parts_mut(jump_table as *mut u32, PROCESS_EVENTS_TABLE_LENGTH as usize)
    };

    let default_offset = jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE];
    PROCESS_EVENTS_DEFAULT_CASE = module.rel_to_abs_addr(default_offset as isize);

    let padding_rel = module.abs_to_rel_addr(padding) as i32;
    patcher.patch(
        (&mut jump_table_slice[SHADER_COMMAND_HIJACKED_TYPE]) as *mut _ as *mut u8,
        &padding_rel.to_ne_bytes(),
    );

    Some(HookState(padding_detour))
}

global_asm!(
    ".global process_events_trampoline",
    "process_events_trampoline:",
    "   MOV rcx, r10",
    "   MOVABS rax, PROCESS_EVENTS_DEFAULT_CASE",
    "   PUSH rax",
    "   JMP process_events_jump_table_9",
);
extern "C" {
    fn process_events_trampoline();
}

#[no_mangle]
unsafe extern "C" fn process_events_jump_table_9(cmd: *const ShaderCommandXIVR) {
    ((*cmd).callback)();
}
