pub mod d3d_payload;
pub mod payload;
pub mod shader_payload;

mod command_stream;
pub use command_stream::*;

use crate::game::graphics::kernel::Texture;
use crate::singleton;

use std::collections::HashMap;
use std::sync::Mutex;

struct InspectedTexture {
    texture: *const Texture,
    width: u32,
    height: u32,
    format: u32,
}

pub struct Debugger {
    pub command_stream: Mutex<CommandStream>,
    inspected_textures: HashMap<*const Texture, InspectedTexture>,
}
singleton!(Debugger);

impl Debugger {
    pub fn new() -> anyhow::Result<Debugger> {
        let command_stream = Mutex::new(CommandStream::new());
        let inspected_textures = HashMap::new();
        Ok(Debugger {
            command_stream,
            inspected_textures,
        })
    }

    pub fn inspect_texture(&mut self, texture: *const Texture) {
        use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;

        let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe {
            (*(*texture).texture_ptr()).GetDesc(&mut desc);
        }

        self.inspected_textures.insert(
            texture,
            InspectedTexture {
                texture,
                width: desc.Width as u32,
                height: desc.Height as u32,
                format: desc.Format.0 as u32,
            },
        );
    }

    fn draw_inspected_texture(&self, tex: &InspectedTexture) -> anyhow::Result<bool> {
        use cimgui as ig;

        let mut open = true;
        let rt_size = ig::Vec2::new(tex.width as f32 / 4.0, tex.height as f32 / 4.0);

        ig::set_next_window_size(
            ig::Vec2::new(rt_size.x, rt_size.y + 150.0),
            Some(ig::Cond::FirstUseEver),
        );
        if ig::begin(
            &format!("Texture {:X?}", tex.texture),
            Some(&mut open),
            None,
        )? {
            use windows::Abi;

            ig::image(
                unsafe { (*(*tex.texture).shader_resource_view_ptr()).abi() },
                rt_size,
                None,
                None,
                None,
                None,
            );

            ig::textf!("Width: {}", tex.width);
            ig::textf!("Height: {}", tex.height);
            ig::textf!("Format: {}", tex.format);

            ig::end();
        }

        Ok(open)
    }

    pub fn draw_render_targets(&mut self) -> anyhow::Result<()> {
        use crate::game::graphics::render;
        use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
        use cimgui as ig;

        let textures = render::RenderTargetManager::get().get_render_targets();
        if ig::begin_table("xivr_debug_tab_rt_list_table", 5, None, None, None)? {
            ig::table_setup_column("Address", None, None, None)?;
            ig::table_setup_column("Offset", None, None, None)?;
            ig::table_setup_column("Width", None, None, None)?;
            ig::table_setup_column("Height", None, None, None)?;
            ig::table_setup_column("Format", None, None, None)?;
            ig::table_headers_row();

            for (offset, texture) in textures.into_iter() {
                let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
                unsafe {
                    (*(*texture).texture_ptr()).GetDesc(&mut desc);
                }

                ig::table_next_row(None, None);
                {
                    ig::table_next_column();
                    if ig::button(&format!("{:X?}", texture), None)? {
                        self.inspect_texture(texture);
                    }
                }
                {
                    ig::table_next_column();
                    ig::textf!("0x{:X}", offset);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{}", desc.Width);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{}", desc.Height);
                }
                {
                    ig::table_next_column();
                    ig::textf!("{:?}", desc.Format);
                }
            }

            ig::end_table();
        }

        Ok(())
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        let mut command_stream = self.command_stream.lock().unwrap();
        command_stream.pre_update()
    }

    pub fn draw_ui(&mut self) -> anyhow::Result<()> {
        use crate::xr::XR;
        use cimgui as ig;

        if ig::begin("XIVR Debugger", None, None)? {
            if ig::begin_tab_bar("xivr_debug_tabs", None)? {
                if ig::begin_tab_item("Command Stream", None, None)? {
                    let mut command_stream = self.command_stream.lock().unwrap();
                    command_stream.draw_ui()?;
                    ig::end_tab_item();
                }
                if ig::begin_tab_item("Render Targets", None, None)? {
                    self.draw_render_targets()?;
                    ig::end_tab_item();
                }
                if let Some(xr) = XR::get_mut() {
                    if ig::begin_tab_item("Framebuffers", None, None)? {
                        xr.draw_ui_framebuffers()?;
                        ig::end_tab_item();
                    }
                    if ig::begin_tab_item("Properties", None, None)? {
                        xr.draw_ui_properties()?;
                        ig::end_tab_item();
                    }
                }
                ig::end_tab_bar();
            }
            ig::end();
        }

        let mut textures_to_remove = vec![];
        for inspected_texture in self.inspected_textures.values() {
            if !self.draw_inspected_texture(&inspected_texture)? {
                textures_to_remove.push(inspected_texture.texture);
            }
        }
        for texture in textures_to_remove {
            self.inspected_textures.remove(&texture);
        }

        Ok(())
    }
}
