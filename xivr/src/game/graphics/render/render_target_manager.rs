use crate::game::graphics::kernel::Texture;
use crate::game::offsets;
use macros::game_class;

game_class!(RenderTargetManager, {
    size: 0x3D8,
    location: offsets::classes::graphics::render::RenderTargetManager::INSTANCES[0],
    fields: {
        [0x070] texture_070: *mut Texture,
        [0x098] texture_098: *mut Texture,
        [0x0B8] texture_0B8: *mut Texture,
        [0x0C0] texture_0C0: *mut Texture,
        [0x0C8] texture_0C8: *mut Texture,
        [0x0D0] texture_0D0: *mut Texture,
        [0x0F8] texture_0F8: *mut Texture,
        [0x160] texture_160: *mut Texture,
        [0x168] texture_168: *mut Texture,
        [0x170] texture_170: *mut Texture,
        [0x190] texture_190: *mut Texture,
        [0x1B0] texture_1B0: *mut Texture,
        [0x1C8] texture_1C8: *mut Texture,
        [0x1D8] texture_1D8: *mut Texture,
        [0x1E0] texture_1E0: *mut Texture,
        [0x200] texture_200: *mut Texture,
        [0x208] texture_208: *mut Texture,
        [0x210] texture_210: *mut Texture,
        [0x218] texture_218: *mut Texture,
        [0x220] texture_220: *mut Texture,
        [0x368] texture_368: *mut Texture,
        [0x370] texture_370: *mut Texture,
        [0x378] texture_378: *mut Texture,
        [0x380] texture_380: *mut Texture,
        [0x388] texture_388: *mut Texture,
        [0x390] texture_390: *mut Texture,
        [0x398] texture_398: *mut Texture,
        [0x3A0] texture_3A0: *mut Texture,
        [0x3A8] texture_3A8: *mut Texture,
        [0x3B0] texture_3B0: *mut Texture,
        [0x3B8] texture_3B8: *mut Texture,
        [0x3C0] texture_3C0: *mut Texture,
        [0x3C8] texture_3C8: *mut Texture,
        [0x3D0] texture_3D0: *mut Texture,
    }
});

impl RenderTargetManager {
    pub fn get_render_targets(&self) -> Vec<(u32, *mut Texture)> {
        let textures = unsafe {
            vec![
                (0x070, *self.texture_070()),
                (0x098, *self.texture_098()),
                (0x0B8, *self.texture_0B8()),
                (0x0C0, *self.texture_0C0()),
                (0x0C8, *self.texture_0C8()),
                (0x0D0, *self.texture_0D0()),
                (0x0F8, *self.texture_0F8()),
                (0x160, *self.texture_160()),
                (0x168, *self.texture_168()),
                (0x170, *self.texture_170()),
                (0x190, *self.texture_190()),
                (0x1B0, *self.texture_1B0()),
                (0x1C8, *self.texture_1C8()),
                (0x1D8, *self.texture_1D8()),
                (0x1E0, *self.texture_1E0()),
                (0x200, *self.texture_200()),
                (0x208, *self.texture_208()),
                (0x210, *self.texture_210()),
                (0x218, *self.texture_218()),
                (0x220, *self.texture_220()),
                (0x368, *self.texture_368()),
                (0x370, *self.texture_370()),
                (0x378, *self.texture_378()),
                (0x380, *self.texture_380()),
                (0x388, *self.texture_388()),
                (0x390, *self.texture_390()),
                (0x398, *self.texture_398()),
                (0x3A0, *self.texture_3A0()),
                (0x3A8, *self.texture_3A8()),
                (0x3B0, *self.texture_3B0()),
                (0x3B8, *self.texture_3B8()),
                (0x3C0, *self.texture_3C0()),
                (0x3C8, *self.texture_3C8()),
                // (0x3D0, *self.texture_3D0()),
            ]
        };

        textures.into_iter().filter(|(_, p)| !p.is_null()).collect()
    }
}
