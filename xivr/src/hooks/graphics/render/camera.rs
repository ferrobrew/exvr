use crate::game::graphics::render;
use crate::util;

use detour::static_detour;

static_detour! {
    pub static Camera_UpdateConstantBuffers_Detour: fn(*mut render::Camera) -> usize;
}

pub struct HookState;
impl Drop for HookState {
    fn drop(&mut self) {
        use crate::log;

        let res = unsafe { Camera_UpdateConstantBuffers_Detour.disable() };
        if let Err(e) = res {
            log!(
                "error",
                "error while disabling constant buffers detour: {}",
                e.to_string()
            )
        }
    }
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use crate::hooks::graphics::kernel::constant_buffer;
    use crate::module::GAME_MODULE;
    use std::mem;

    let module = GAME_MODULE
        .get()
        .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?;

    let camera_updateconstantbuffers_addr =
        module.scan_for_relative_callsite("E8 ? ? ? ? E9 ? ? ? ? 42 83 64 37 ? ?")?;
    constant_buffer::RENDER_CAMERA_UPDATE_CONSTANT_BUFFERS_PTR =
        Some(camera_updateconstantbuffers_addr);

    Camera_UpdateConstantBuffers_Detour.initialize(
        mem::transmute(camera_updateconstantbuffers_addr),
        move |s| {
            util::handle_error_in_block(|| {
                if s == *render::RenderManager::get().render_camera() {
                    // The render camera is rendering. Let's back up its state to our friendly
                    // constant buffer hook so that it can use it to calculate the per-eye matrices!
                    unsafe {
                        constant_buffer::RENDER_CAMERA_BUFFER = Some((*s).clone());
                    }
                }
                Camera_UpdateConstantBuffers_Detour.call(s);
                Ok(0usize)
            })
        },
    )?;
    Camera_UpdateConstantBuffers_Detour.enable()?;

    Ok(HookState {})
}
