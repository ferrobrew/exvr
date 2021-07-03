pub mod patcher;
pub use patcher::*;

pub mod graphics;

pub struct HookState {
    _kernel_context: graphics::kernel::context::HookState,
    _render_render_manager: graphics::render::render_manager::HookState,
}

impl HookState {
    pub fn new(module: &crate::module::Module, patcher: &mut Patcher) -> Option<HookState> {
        Some(HookState {
            _kernel_context: unsafe {
                graphics::kernel::context::patch_process_events(module, patcher)?
            },
            _render_render_manager: unsafe {
                graphics::render::render_manager::hook_rendermanager_render(module)?
            },
        })
    }
}
