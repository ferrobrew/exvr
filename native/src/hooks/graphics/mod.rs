use crate::hooks::Patcher;

pub mod kernel;
pub mod render;

pub struct HookState {
    _kernel_context: kernel::context::HookState,
    _render_render_manager: render::render_manager::HookState,
}

impl HookState {
    pub fn new(patcher: &mut Patcher) -> Option<HookState> {
        Some(HookState {
            _kernel_context: unsafe { kernel::context::patch_process_events(patcher)? },
            _render_render_manager: unsafe { render::render_manager::hook_rendermanager_render()? },
        })
    }
}
