use crate::game::graphics::kernel::Device;
use crate::hooks::graphics::kernel::context::ShaderCommandXIVR;
use crate::log;
use crate::module::{Module, GAME_MODULE};

use detour::static_detour;
use std::mem;

static_detour! {
    pub static RenderManager_Render_Detour: fn(usize) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { RenderManager_Render_Detour.disable() };
        if let Err(e) = res {
            log!("error while disabling context detour: {}", e.to_string())
        }
    }
}

pub unsafe fn hook_rendermanager_render() -> Option<HookState> {
    let module = GAME_MODULE.get()?;

    let tls_index = get_tls_index(&module);
    let rendermanager_render_addr = module.scan("40 53 55 57 41 56 41 57 48 83 EC 60")?;
    let context_alloc: fn(*const u8, usize) -> *const u8 =
        mem::transmute(module.scan("4C 8D 4A 0F 4C 8B C1")?);
    let context_pushbackevent: fn(*const u8, *const ShaderCommandXIVR) -> usize =
        mem::transmute(module.scan("83 41 30 FF")?);

    RenderManager_Render_Detour
        .initialize(mem::transmute(rendermanager_render_addr), move |s| {
            let ret = RenderManager_Render_Detour.call(s);
            let rc = get_render_context(tls_index);
            let cmd =
                context_alloc(rc, mem::size_of::<ShaderCommandXIVR>()) as *mut ShaderCommandXIVR;
            *cmd = ShaderCommandXIVR::new(|| {
            });
            context_pushbackevent(rc, cmd);
            ret
        })
        .ok()?;

    RenderManager_Render_Detour.enable().ok()?;

    Some(HookState {})
}

fn get_tls_index(module: &Module) -> u32 {
    struct TlsDirectory {
        _tls_start: *const u8,
        _tls_end: *const u8,
        tls_index: *const u32,
        // rest elided
    }

    unsafe {
        let dir_offset = module.rel_to_abs_addr(0x240) as *const u32;
        let dir = module.rel_to_abs_addr((*dir_offset) as isize) as *const TlsDirectory;
        *((*dir).tls_index)
    }
}

#[naked]
extern "C" fn get_render_context(_tls_index: u32) -> *const u8 {
    unsafe {
        asm! {
            "MOV rax, gs:58h",
            "MOV rax, [rax+rcx*8]",
            "MOV rcx, 250h",
            "MOV rax, [rax+rcx]",
            "RET",
            options(noreturn)
        }
    }
}
