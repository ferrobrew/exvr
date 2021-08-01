pub mod d3d_payload;
pub mod payload;
pub mod shader_payload;

mod util;

mod command_stream;
pub use command_stream::*;

use crate::game::graphics::kernel::{Device, Texture};
use crate::singleton;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
use util::dxgi_format_to_str;

#[derive(PartialEq, Eq)]
enum InspectedResource {
    Texture(d3d::ID3D11Texture2D, d3d::ID3D11ShaderResourceView),
}

impl Hash for InspectedResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use windows::Abi;
        match self {
            InspectedResource::Texture(tex, ..) => tex.abi().hash(state),
        }
    }
}

pub struct Debugger {
    pub command_stream: Mutex<CommandStream>,
    inspected_textures: HashSet<&'static Texture>,
    inspected_resources: HashSet<InspectedResource>,
    some_global_struct: *const u8,
}
singleton!(Debugger);

impl Debugger {
    pub fn new() -> anyhow::Result<Debugger> {
        use crate::module::GAME_MODULE;

        let command_stream = Mutex::new(CommandStream::new());
        let inspected_textures = HashSet::new();
        let inspected_resources = HashSet::new();

        let module = unsafe {
            GAME_MODULE
                .get()
                .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?
        };

        let mystery_function: fn() -> *const u8 = unsafe {
            std::mem::transmute(module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 58 60")?)
        };
        let some_global_struct = mystery_function();

        Ok(Debugger {
            command_stream,
            inspected_textures,
            inspected_resources,
            some_global_struct,
        })
    }

    pub fn inspect_texture(&mut self, texture: &'static Texture) {
        self.inspected_textures.insert(texture);
    }

    pub fn inspect_resource(&mut self, resource: d3d::ID3D11Resource) -> anyhow::Result<()> {
        use windows::Interface;
        use windows::Abi;
        if let Ok(tex) = resource.cast::<d3d::ID3D11Texture2D>() {
            let device = unsafe { Device::get().device() };

            let desc = unsafe {
                let mut desc: d3d::D3D11_TEXTURE2D_DESC = std::mem::zeroed();
                tex.GetDesc(&mut desc);
                desc
            };

            let srv_desc = d3d::D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: desc.Format,
                ViewDimension: d3d::D3D_SRV_DIMENSION_TEXTURE2D,
                Anonymous: d3d::D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: d3d::D3D11_TEX2D_SRV {
                        MostDetailedMip: 0,
                        MipLevels: desc.MipLevels,
                    },
                },
            };
            let srv = unsafe { device.CreateShaderResourceView(tex.clone(), &srv_desc)? };

            self.inspected_resources
                .insert(InspectedResource::Texture(tex, srv));
        }

        Ok(())
    }

    fn draw_inspected_texture_internal(
        tex: d3d::ID3D11Texture2D,
        srv: d3d::ID3D11ShaderResourceView,
    ) -> anyhow::Result<bool> {
        use cimgui as ig;
        use windows::Abi;

        let mut open = true;
        let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe {
            tex.GetDesc(&mut desc);
        }

        let base_width = desc.Width as f32 / 2.0;
        let inverse_aspect_ratio = desc.Height as f32 / desc.Width as f32;
        ig::set_next_window_size(
            ig::Vec2::new(base_width, base_width * inverse_aspect_ratio + 40.0),
            Some(ig::Cond::Once),
        );
        ig::set_next_window_bg_alpha(1.0);
        if ig::begin(
            &format!(
                "Texture {:X?} ({}x{}, {})",
                tex.abi(),
                desc.Width,
                desc.Height,
                dxgi_format_to_str(desc.Format)
            ),
            Some(&mut open),
            None,
        )? {
            let ig::Vec2 { x: width, .. } = ig::get_window_size();
            let size = ig::Vec2::new(width, width * inverse_aspect_ratio);
            ig::image(srv.abi(), size, None, None, None, None);
            ig::bulletf!("Texture pointer: {:X?}", tex.abi());
            ig::end();
        }

        Ok(open)
    }

    fn draw_inspected_texture(tex: &Texture) -> anyhow::Result<bool> {
        unsafe {
            Self::draw_inspected_texture_internal(
                tex.texture().clone().into(),
                tex.shader_resource_view().clone().into(),
            )
        }
    }

    fn draw_inspected_resource(res: &InspectedResource) -> anyhow::Result<bool> {
        match res {
            InspectedResource::Texture(tex, srv) => {
                Self::draw_inspected_texture_internal(tex.clone().into(), srv.clone().into())
            }
        }
    }

    fn draw_render_target(
        &mut self,
        description: &str,
        texture: &'static Texture,
    ) -> anyhow::Result<()> {
        use cimgui as ig;

        let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe {
            texture.texture().GetDesc(&mut desc);
        }

        const PREVIEW_HEIGHT: f32 = 64.0;
        ig::table_next_row(None, None);
        {
            use windows::Abi;
            let aspect_ratio = desc.Width as f32 / desc.Height as f32;
            let rt_size = ig::Vec2::new(aspect_ratio * PREVIEW_HEIGHT, PREVIEW_HEIGHT);

            ig::table_next_column();
            let img = unsafe { texture.shader_resource_view().abi() };
            if ig::image_button(
                img,
                rt_size,
                None,
                None,
                None,
                Some(ig::Color::new(0.0, 0.0, 0.0, 1.0)),
                None,
            ) {
                self.inspect_texture(texture);
            }
        }
        {
            ig::table_next_column();
            ig::textf!("{:X?}", texture as *const _);
        }
        {
            ig::table_next_column();
            ig::text(description);
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
            ig::textf!("{}", dxgi_format_to_str(desc.Format));
        }

        Ok(())
    }

    pub fn draw_render_targets(&mut self) -> anyhow::Result<()> {
        use crate::game::graphics::{kernel, render};
        use cimgui as ig;

        let setup_columns = |headers| -> anyhow::Result<()> {
            for header in headers {
                ig::table_setup_column(header, None, None, None)?;
            }
            ig::table_headers_row();
            Ok(())
        };

        if ig::begin_child("xivr_debug_tab_rts_child", None, None, None)? {
            if ig::collapsing_header("Mystery structure", None, None)? {
                let texture_ptr: &Texture = unsafe {
                    let some_struct = *(self.some_global_struct.add(0x60) as *const *const u8);
                    &**(some_struct.add(0x10) as *const *const Texture)
                };

                if ig::begin_table("xivr_debug_tab_rts_swapchain", 6, None, None, None)? {
                    setup_columns(["Preview", "Address", "Title", "Width", "Height", "Format"])?;

                    self.draw_render_target("Backbuffer (real?)", texture_ptr)?;

                    ig::end_table();
                }
            }

            if ig::collapsing_header("Swapchain", None, None)? {
                let swapchain = unsafe { &*kernel::Device::get().swapchain_ptr() };
                if ig::begin_table("xivr_debug_tab_rts_swapchain", 6, None, None, None)? {
                    setup_columns(["Preview", "Address", "Title", "Width", "Height", "Format"])?;

                    self.draw_render_target("Backbuffer", unsafe {
                        &*(*swapchain.back_buffer() as *const _)
                    })?;

                    ig::end_table();
                }
            }

            if ig::collapsing_header("Render Target Manager", None, None)? {
                let textures = unsafe { render::RenderTargetManager::get().get_render_targets() };
                if ig::begin_table("xivr_debug_tab_rts_rtm", 6, None, None, None)? {
                    setup_columns(["Preview", "Address", "Offset", "Width", "Height", "Format"])?;

                    for (offset, texture) in textures.into_iter() {
                        self.draw_render_target(&format!("0x{:X}", offset), unsafe { &*texture })?;
                    }

                    ig::end_table();
                }
            }
            ig::end_child();
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
        // It seems a little dubious to me to mutate inside a method like this, but if it works, it works
        self.inspected_textures
            .retain(|t| Self::draw_inspected_texture(t).unwrap_or(false));
        self.inspected_resources
            .retain(|t| Self::draw_inspected_resource(t).unwrap_or(false));

        Ok(())
    }
}
