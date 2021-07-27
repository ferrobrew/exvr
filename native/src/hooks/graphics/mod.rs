pub mod d3d;
pub mod kernel;
pub mod render;

pub struct HookState {
    _d3d_device_context: d3d::device_context::HookState,
    _kernel_context: kernel::context::HookState,
    _kernel_immediate_context: kernel::immediate_context::HookState,
    _render_render_manager: render::render_manager::HookState,
}

impl HookState {
    pub fn new() -> anyhow::Result<HookState> {
        unsafe {
            Ok(HookState {
                _d3d_device_context: d3d::device_context::install()?,
                _kernel_context: kernel::context::install()?,
                _kernel_immediate_context: kernel::immediate_context::install()?,
                _render_render_manager: render::render_manager::install()?,
            })
        }
    }
}
