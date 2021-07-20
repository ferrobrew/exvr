use crate::game;
use macros::game_class;

game_class!(ShaderCommandPayloadSetRenderTargets, {
	attributes: #[derive(Copy, Clone)],
	size: 0x34,
	fields: {
		[0x0] render_target_count: u32,
		[0x4] render_targets: [*const game::graphics::kernel::Texture; 4],
	}
});

impl ShaderCommandPayloadSetRenderTargets {
	pub fn get_render_target_slice(&self) -> &[*const game::graphics::kernel::Texture] {
		unsafe {
			std::slice::from_raw_parts(
				std::mem::transmute(self.render_targets_ptr()),
				self.render_target_count as usize,
			)
		}
	}
}

game_class!(ShaderCommandPayloadCopyTexture, {
	attributes: #[derive(Copy, Clone)],
	size: 0x44,
	fields: {
		[0x4] dst_resource: *const game::graphics::kernel::Texture,
		[0x1C] src_resource: *const game::graphics::kernel::Texture,
	}
});

pub union ShaderCommandPayload {
	pub set_render_targets: std::mem::ManuallyDrop<ShaderCommandPayloadSetRenderTargets>,
	pub copy_texture: std::mem::ManuallyDrop<ShaderCommandPayloadCopyTexture>,
}

#[repr(u32)]
#[allow(dead_code)]
pub enum ShaderCommandType {
	SetRenderTargets = 0,
	SetViewports = 1,
	SetViewportsFancy = 2,
	SetScissorRect = 3,
	Clear = 4,
	Draw = 5,
	DrawIndexed = 6,
	DrawIndexedInstanced = 7,
	DispatchComputeShader = 8,
	XIVRHijack = 9,
	CopyTexture = 10,
	UnknownDraw = 11,
	CopyResource = 12,
	ResetRendererMaybe = 13,
	Unknown1 = 14,
	CopySubresourceRegion = 16,
	SomethingWithStrings = 17,
}

game_class!(ShaderCommand, {
	fields: {
		[0x0] cmd_type: ShaderCommandType,
		[0x4] payload: ShaderCommandPayload,
	}
});
