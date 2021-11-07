use crate::game::graphics::render;
use crate::game::math;
use crate::module::GAME_MODULE;
use crate::{log, util};

use detour::static_detour;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct CameraParameters {
    view_matrix: math::Mat3x4,
    inverse_view_matrix: math::Mat3x4,
    view_projection_matrix: math::Mat4,
    inverse_view_projection_matrix: math::Mat4,
    inverse_projection_matrix: math::Mat4,
    projection_matrix: math::Mat4,
    main_view_to_projection_matrix: math::Mat4,
    eye_position: math::Vec3A,
    look_at_vector: math::Vec3A,
}
static_assertions::const_assert!(std::mem::size_of::<CameraParameters>() == 448);

const CAMERA_PARAMETERS_SIZE: usize = 8;
const CAMERA_PARAMETERS_INIT: Option<Box<CameraParameters>> = None;
// A queue of pointers to parameters. Kept alive as there may be some delay between when the
// parameters are created and when they're used.
static mut CAMERA_PARAMETERS_PTRS: [Option<Box<CameraParameters>>; CAMERA_PARAMETERS_SIZE] =
    [CAMERA_PARAMETERS_INIT; CAMERA_PARAMETERS_SIZE];

// Written to by the Camera hook, and used to call the function that calculates matrices for
// a camera. It's a bit of an overkill solution, but I didn't want to reimplement all of that
// logic just to submit the per-eye constant buffers...
pub static mut RENDER_CAMERA_BUFFER: Option<render::Camera> = None;
pub static mut RENDER_CAMERA_UPDATE_CONSTANT_BUFFERS_PTR: Option<*mut u8> = None;

#[repr(C, packed)]
pub struct BufferLoad {
    data: *const u8,
    size: u32,
    _pad: u32,
}
static_assertions::const_assert!(std::mem::size_of::<BufferLoad>() == 16);

static_detour! {
    pub static ConstantBuffer_LoadBuffer_Detour: fn(usize, usize, usize, *const BufferLoad);
}

pub struct HookState;

impl Drop for HookState {
    fn drop(&mut self) {
        let res = unsafe { ConstantBuffer_LoadBuffer_Detour.disable() };
        if let Err(e) = res {
            log!("error", "error while disabling loadbuffer hijack: {}", e.to_string());
        }
    }
}

unsafe fn loadbuffer_implementation(
    this: usize,
    unk1: usize,
    unk2: usize,
    load_ptr: *const BufferLoad,
    size: u32,
) -> Option<()> {
    let camera = RENDER_CAMERA_BUFFER.as_ref()?;

    let affine3a_to_mat4 = |val: glam::Affine3A| {
        let mut out = glam::Mat4::from_mat3(val.matrix3.into());
        let translation: glam::Vec3 = val.translation.into();
        *out.col_mut(3) = (translation, 0.0).into();
        out
    };

    #[rustfmt::skip]
    let transpose_4x4_to_3x4 = |val: glam::Mat4| {
        let val = val.as_ref();
        math::Mat3x4([
            val[0],  val[4],  val[8],  val[12],
            val[1],  val[5],  val[9],  val[13],
            val[2],  val[6],  val[10], val[14],
        ])
    };

    let transpose_3x4 = |val: glam::Affine3A| transpose_4x4_to_3x4(affine3a_to_mat4(val));

    let view_matrix: glam::Affine3A = (*camera.view_matrix()).into();

    let old: &CameraParameters = &*(std::ptr::read_unaligned(std::ptr::addr_of!((*load_ptr).data))
        as *const CameraParameters);

    let projection_matrix: glam::Mat4 = (*camera.projection_matrix()).into();
    let view_projection_matrix: glam::Mat4 = view_matrix * projection_matrix;
    let eye_position: glam::Vec3A = (*camera.eye_position()).into();
    let inverse_view_matrix = view_matrix.inverse();

    let temp_buffer = Box::new(CameraParameters {
        view_matrix: transpose_3x4(view_matrix),
        inverse_view_matrix: transpose_3x4(inverse_view_matrix),
        view_projection_matrix: view_projection_matrix.transpose().into(),
        inverse_view_projection_matrix: view_projection_matrix.inverse().transpose().into(),
        inverse_projection_matrix: projection_matrix.inverse().transpose().into(),
        projection_matrix: projection_matrix.transpose().into(),
        // main_view_to_projection_matrix: glam::Mat4::IDENTITY.into(),
        // main_view_to_projection_matrix: projection_matrix.transpose().into(),
        eye_position: eye_position.into(),
        look_at_vector: inverse_view_matrix.matrix3.z_axis.normalize().into(),
        ..*old
    });

    let data = &*temp_buffer as *const _ as *const u8;
    let _pad = 0;
    let new_load = BufferLoad { data, size, _pad };
    let new_load_ptr = &new_load as *const BufferLoad;

    CAMERA_PARAMETERS_PTRS.rotate_left(1);
    *CAMERA_PARAMETERS_PTRS.last_mut()? = Some(temp_buffer);

    ConstantBuffer_LoadBuffer_Detour.call(this, unk1, unk2, new_load_ptr);

    Some(())
}

fn constantbuffer_loadbuffer_hook(this: usize, unk1: usize, unk2: usize, load: *const BufferLoad) {
    util::handle_error_in_block(move || {
        use crate::debugger::Debugger;
        use std::ptr;

        let size = unsafe { ptr::read_unaligned(ptr::addr_of!((*load).size)) };
        if size as usize == std::mem::size_of::<CameraParameters>() / 16 {
            if unsafe { loadbuffer_implementation(this, unk1, unk2, load, size).is_some() } {
                // We successfully hijacked this load, return
                return Ok(());
            }
        }

        if let Some(debugger) = Debugger::get_mut() {
            if let Ok(mut command_stream) = debugger.command_stream.lock() {
                let data = unsafe { ptr::read_unaligned(ptr::addr_of!((*load).data)) };
                let submsgs = (0..size)
                    .map(|i| {
                        format!("{:?}", unsafe {
                            std::slice::from_raw_parts(
                                (data as *const f32).add((i * 4) as usize),
                                4,
                            )
                        })
                    })
                    .collect();

                command_stream
                    .add_message(format!("CB{:X?} <- {} bytes", this, size * 16), submsgs)?;
            }
        }

        ConstantBuffer_LoadBuffer_Detour.call(this, unk1, unk2, load);
        Ok(())
    });
}

pub unsafe fn install() -> anyhow::Result<HookState> {
    use std::mem;

    let module = GAME_MODULE
        .get()
        .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?;
    let constantbuffer_loadbuffer = module.scan("4C 89 44 24 ? 56 57 41 57")?;

    ConstantBuffer_LoadBuffer_Detour.initialize(
        mem::transmute(constantbuffer_loadbuffer),
        constantbuffer_loadbuffer_hook,
    )?;
    ConstantBuffer_LoadBuffer_Detour.enable()?;

    Ok(HookState {})
}
