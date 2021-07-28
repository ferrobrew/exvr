use crate::ct_config::rendering::SHADER_COMMAND_HIJACKED_TYPE;
use crate::game;
use macros::game_class;

/* Port the following at some point:
struct
{
	int bounds[4];
	float minDepth;
	float maxDepth;
} SetViewport;
static_assert(sizeof(SetViewport) + sizeof(ShaderCommandType) == 0x1C);

struct
{
	D3D11_RECT rect;
} SetScissorRect;
static_assert(sizeof(SetScissorRect) + sizeof(ShaderCommandType) == 0x14);

struct
{
	int clearFlags;
	float colour[4];
	double field_18;
	uint64_t field_20;
	int field_28;
	int field_2C;
	int field_30;
	int field_34;
} Clear;
static_assert(sizeof(Clear) + sizeof(ShaderCommandType) == 0x38);

struct
{
	int field_4;
	int startVertexLocation;
	uint32_t vertexCount;
	uint8_t probablyModel[0x70];
} Draw;
static_assert(sizeof(Draw) + sizeof(ShaderCommandType) == 0x80);

struct
{
	int field_4;
	uint32_t baseVertexLocation;
	int field_C;
	int field_10;
	uint32_t startIndexLocation;
	uint32_t indexCount;
	int field_1C;
	int field_20;
	int field_24;
	int field_28;
	int field_2C;
	int field_30;
	int field_34;
	int field_38;
	int field_3C;
	int field_40;
	int field_44;
	int field_48;
	int field_4C;
	int field_50;
	int field_54;
	int field_58;
	int field_5C;
	int field_60;
	int field_64;
	int field_68;
	int field_6C;
	int field_70;
	int field_74;
	int field_78;
	uint64_t field_7C;
	uint64_t field_84;
} DrawIndexed;
static_assert(sizeof(DrawIndexed) + sizeof(ShaderCommandType) == 0x8C);

struct
{
	int field_4;
	int baseVertexLocation;
	int field_C;
	uint32_t field_10;
	int startIndexLocation;
	int indexCountPerInstance;
	int instanceCount;
	uint8_t probablyModel[0x70];
} DrawIndexedInstanced;
static_assert(sizeof(DrawIndexedInstanced) + sizeof(ShaderCommandType) == 0x90);

struct
{
	int field_4;
	uint64_t field_8;
	uint64_t field_10;
	uint64_t field_18;
	uint64_t field_20;
	uint32_t threadGroupCountX;
	uint32_t threadGroupCountY;
	uint32_t threadGroupCountZ;
	int field_34;
} DispatchComputeShader;
static_assert(sizeof(DispatchComputeShader) + sizeof(ShaderCommandType) == 0x38);

struct
{
	int field_4;
	game::graphics::kernel::Texture* dstResource;
	uint32_t dstSubresource;
	uint32_t dstXY[2];
	int field_1C;
	game::graphics::kernel::Texture* srcResource;
	int srcSubresource;
	int field_2C;
	uint64_t useSuppliedRect;
	uint32_t rect[4];
} CopyTexture;
static_assert(sizeof(CopyTexture) + sizeof(ShaderCommandType) == 0x48);

struct
{
	int field_4;
	game::graphics::kernel::Texture* texture;
	int field_10;
	int field_14;
	uint64_t field_18;
	uint64_t field_20;
	int field_28;
	int field_2C;
	uint64_t field_30;
	int field_38;
	int field_3C;
	int field_40;
	int field_44;
	int field_48;
	int field_4C;
	int field_50;
	int field_54;
} UnknownDraw;
static_assert(sizeof(UnknownDraw) + sizeof(ShaderCommandType) == 0x58);

struct
{
	int field_4;
	uint64_t callback;
	uint64_t field_10;
	int field_18;
	int field_1C;
} ResetRendererMaybe;
static_assert(sizeof(ResetRendererMaybe) + sizeof(ShaderCommandType) == 0x20);
*/

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
#[derive(PartialEq, Copy, Clone, Debug)]
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
	XIVRHijack = SHADER_COMMAND_HIJACKED_TYPE as u32,
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
