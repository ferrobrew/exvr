pub mod kernel;
pub mod render;

pub struct HookState {
    _kernel_context: kernel::context::HookState,
    _kernel_immediate_context: kernel::immediate_context::HookState,
    _render_render_manager: render::render_manager::HookState,
}

impl HookState {
    pub fn new() -> anyhow::Result<HookState> {
        unsafe {
            Ok(HookState {
                _kernel_context: kernel::context::install()?,
                _kernel_immediate_context: kernel::immediate_context::install()?,
                _render_render_manager: render::render_manager::install()?,
            })
        }
    }
}
