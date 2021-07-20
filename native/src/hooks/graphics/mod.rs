use crate::hooks::Patcher;

pub mod kernel;
pub mod render;

pub struct HookState {
    _kernel_context: kernel::context::HookState,
    _kernel_immediate_context: kernel::immediate_context::HookState,
    _render_render_manager: render::render_manager::HookState,
}

impl HookState {
    pub fn new(patcher: &mut Patcher) -> Option<HookState> {
        unsafe {
            Some(HookState {
                _kernel_context: kernel::context::install(patcher)?,
                _kernel_immediate_context: kernel::immediate_context::install(patcher)?,
                _render_render_manager: render::render_manager::install()?,
            })
        }
    }
}
