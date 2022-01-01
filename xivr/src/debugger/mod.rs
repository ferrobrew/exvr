pub mod d3d_payload;
pub mod message_payload;
pub mod payload;
pub mod shader_payload;

mod util;

mod command_stream;
pub use command_stream::*;

use crate::debugger::util::dxgi_format_to_str;
use crate::game::graphics::kernel::{Device, Texture};
use crate::singleton;

use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Mutex;

use windows::Win32::Graphics::Direct3D11 as d3d;

#[derive(PartialEq, Eq)]
enum InspectedResource {
    Texture(d3d::ID3D11Texture2D, Option<d3d::ID3D11ShaderResourceView>),
}

#[allow(clippy::derive_hash_xor_eq)]
impl Hash for InspectedResource {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            InspectedResource::Texture(tex, ..) => {
                let ptr: *mut () = unsafe { std::mem::transmute(tex) };
                ptr.hash(state)
            }
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
        let command_stream = Mutex::new(CommandStream::new());
        let inspected_textures = HashSet::new();
        let inspected_resources = HashSet::new();

        let module = crate::util::game_module_mut()?;
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

    pub fn inspect_d3d_resource(&mut self, resource: d3d::ID3D11Resource) -> anyhow::Result<()> {
        use windows::runtime::Interface;
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
            let srv = unsafe { device.CreateShaderResourceView(tex.clone(), &srv_desc).ok() };
            self.inspect_d3d_texture(tex, srv)?;
        }

        Ok(())
    }

    pub fn inspect_d3d_texture(
        &mut self,
        tex: d3d::ID3D11Texture2D,
        srv: Option<d3d::ID3D11ShaderResourceView>,
    ) -> anyhow::Result<()> {
        self.inspected_resources
            .insert(InspectedResource::Texture(tex, srv));
        Ok(())
    }

    fn draw_inspected_texture_internal(
        tex: d3d::ID3D11Texture2D,
        srv: Option<d3d::ID3D11ShaderResourceView>,
    ) -> anyhow::Result<bool> {
        use cimgui as ig;

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
                tex,
                desc.Width,
                desc.Height,
                dxgi_format_to_str(desc.Format)
            ),
            Some(&mut open),
            None,
        )? {
            let ig::Vec2 { x: width, .. } = ig::get_window_size();
            if let Some(srv) = srv {
                let size = ig::Vec2::new(width, width * inverse_aspect_ratio);
                ig::image(
                    unsafe { std::mem::transmute(srv) },
                    size,
                    None,
                    None,
                    None,
                    None,
                );
            } else {
                ig::text("Unable to bind texture to SRV");
            }
            ig::bulletf!("Texture pointer: {:X?}", tex);
            ig::bulletf!("Width: {}", desc.Width);
            ig::bulletf!("Height: {}", desc.Height);
            ig::bulletf!("Array Size: {}", desc.ArraySize);
            ig::bulletf!("Mip Levels: {}", desc.MipLevels);
            ig::bulletf!("Sample Count: {}", desc.SampleDesc.Count);
            ig::bulletf!("Format: {}", dxgi_format_to_str(desc.Format));
            ig::bulletf!("Bind Flags: {}", desc.BindFlags.0);
            ig::end();
        }

        Ok(open)
    }

    fn draw_inspected_texture(tex: &Texture) -> anyhow::Result<bool> {
        unsafe {
            let srv = tex.shader_resource_view();

            if let Some(srv) = srv {
                Self::draw_inspected_texture_internal(
                    tex.texture().clone(),
                    Some(srv.clone().into()),
                )
            } else {
                Ok(false)
            }
        }
    }

    fn draw_inspected_resource(res: &InspectedResource) -> anyhow::Result<bool> {
        match res {
            InspectedResource::Texture(tex, srv) => {
                Self::draw_inspected_texture_internal(tex.clone(), srv.as_ref().cloned())
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
            let aspect_ratio = desc.Width as f32 / desc.Height as f32;
            let rt_size = ig::Vec2::new(aspect_ratio * PREVIEW_HEIGHT, PREVIEW_HEIGHT);

            ig::table_next_column();
            let img = unsafe { std::mem::transmute(texture.shader_resource_view()) };
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
        use crate::game::graphics::kernel;
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
                let texture: &Texture = unsafe {
                    let some_struct = *(self.some_global_struct.add(0x60) as *const *const u8);
                    &**(some_struct.add(0x10) as *const *const Texture)
                };

                if ig::begin_table("xivr_debug_tab_rts_mystery", 6, None, None, None)? {
                    setup_columns(["Preview", "Address", "Title", "Width", "Height", "Format"])?;

                    self.draw_render_target("Backbuffer (real?)", texture)?;

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
