pub mod d3d_payload;
pub mod payload;
pub mod shader_payload;

mod command_stream;
pub use command_stream::*;

use crate::game::graphics::kernel::Texture;
use crate::singleton;

use std::collections::HashMap;
use std::sync::Mutex;

use bindings::Windows::Win32::Graphics::Dxgi as dxgi;

struct InspectedTexture {
    texture: *const Texture,
    width: u32,
    height: u32,
    format: dxgi::DXGI_FORMAT,
}

pub struct Debugger {
    pub command_stream: Mutex<CommandStream>,
    inspected_textures: HashMap<*const Texture, InspectedTexture>,
    some_global_struct: *const u8,
}
singleton!(Debugger);

fn dxgi_format_to_str(dxgi_format: dxgi::DXGI_FORMAT) -> &'static str {
    match dxgi_format {
        dxgi::DXGI_FORMAT_R32G32B32A32_TYPELESS => "R32G32B32A32_TYPELESS",
        dxgi::DXGI_FORMAT_R32G32B32A32_FLOAT => "R32G32B32A32_FLOAT",
        dxgi::DXGI_FORMAT_R32G32B32A32_UINT => "R32G32B32A32_UINT",
        dxgi::DXGI_FORMAT_R32G32B32A32_SINT => "R32G32B32A32_SINT",
        dxgi::DXGI_FORMAT_R32G32B32_TYPELESS => "R32G32B32_TYPELESS",
        dxgi::DXGI_FORMAT_R32G32B32_FLOAT => "R32G32B32_FLOAT",
        dxgi::DXGI_FORMAT_R32G32B32_UINT => "R32G32B32_UINT",
        dxgi::DXGI_FORMAT_R32G32B32_SINT => "R32G32B32_SINT",
        dxgi::DXGI_FORMAT_R16G16B16A16_TYPELESS => "R16G16B16A16_TYPELESS",
        dxgi::DXGI_FORMAT_R16G16B16A16_FLOAT => "R16G16B16A16_FLOAT",
        dxgi::DXGI_FORMAT_R16G16B16A16_UNORM => "R16G16B16A16_UNORM",
        dxgi::DXGI_FORMAT_R16G16B16A16_UINT => "R16G16B16A16_UINT",
        dxgi::DXGI_FORMAT_R16G16B16A16_SNORM => "R16G16B16A16_SNORM",
        dxgi::DXGI_FORMAT_R16G16B16A16_SINT => "R16G16B16A16_SINT",
        dxgi::DXGI_FORMAT_R32G32_TYPELESS => "R32G32_TYPELESS",
        dxgi::DXGI_FORMAT_R32G32_FLOAT => "R32G32_FLOAT",
        dxgi::DXGI_FORMAT_R32G32_UINT => "R32G32_UINT",
        dxgi::DXGI_FORMAT_R32G32_SINT => "R32G32_SINT",
        dxgi::DXGI_FORMAT_R32G8X24_TYPELESS => "R32G8X24_TYPELESS",
        dxgi::DXGI_FORMAT_D32_FLOAT_S8X24_UINT => "D32_FLOAT_S8X24_UINT",
        dxgi::DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS => "R32_FLOAT_X8X24_TYPELESS",
        dxgi::DXGI_FORMAT_X32_TYPELESS_G8X24_UINT => "X32_TYPELESS_G8X24_UINT",
        dxgi::DXGI_FORMAT_R10G10B10A2_TYPELESS => "R10G10B10A2_TYPELESS",
        dxgi::DXGI_FORMAT_R10G10B10A2_UNORM => "R10G10B10A2_UNORM",
        dxgi::DXGI_FORMAT_R10G10B10A2_UINT => "R10G10B10A2_UINT",
        dxgi::DXGI_FORMAT_R11G11B10_FLOAT => "R11G11B10_FLOAT",
        dxgi::DXGI_FORMAT_R8G8B8A8_TYPELESS => "R8G8B8A8_TYPELESS",
        dxgi::DXGI_FORMAT_R8G8B8A8_UNORM => "R8G8B8A8_UNORM",
        dxgi::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB => "R8G8B8A8_UNORM_SRGB",
        dxgi::DXGI_FORMAT_R8G8B8A8_UINT => "R8G8B8A8_UINT",
        dxgi::DXGI_FORMAT_R8G8B8A8_SNORM => "R8G8B8A8_SNORM",
        dxgi::DXGI_FORMAT_R8G8B8A8_SINT => "R8G8B8A8_SINT",
        dxgi::DXGI_FORMAT_R16G16_TYPELESS => "R16G16_TYPELESS",
        dxgi::DXGI_FORMAT_R16G16_FLOAT => "R16G16_FLOAT",
        dxgi::DXGI_FORMAT_R16G16_UNORM => "R16G16_UNORM",
        dxgi::DXGI_FORMAT_R16G16_UINT => "R16G16_UINT",
        dxgi::DXGI_FORMAT_R16G16_SNORM => "R16G16_SNORM",
        dxgi::DXGI_FORMAT_R16G16_SINT => "R16G16_SINT",
        dxgi::DXGI_FORMAT_R32_TYPELESS => "R32_TYPELESS",
        dxgi::DXGI_FORMAT_D32_FLOAT => "D32_FLOAT",
        dxgi::DXGI_FORMAT_R32_FLOAT => "R32_FLOAT",
        dxgi::DXGI_FORMAT_R32_UINT => "R32_UINT",
        dxgi::DXGI_FORMAT_R32_SINT => "R32_SINT",
        dxgi::DXGI_FORMAT_R24G8_TYPELESS => "R24G8_TYPELESS",
        dxgi::DXGI_FORMAT_D24_UNORM_S8_UINT => "D24_UNORM_S8_UINT",
        dxgi::DXGI_FORMAT_R24_UNORM_X8_TYPELESS => "R24_UNORM_X8_TYPELESS",
        dxgi::DXGI_FORMAT_X24_TYPELESS_G8_UINT => "X24_TYPELESS_G8_UINT",
        dxgi::DXGI_FORMAT_R8G8_TYPELESS => "R8G8_TYPELESS",
        dxgi::DXGI_FORMAT_R8G8_UNORM => "R8G8_UNORM",
        dxgi::DXGI_FORMAT_R8G8_UINT => "R8G8_UINT",
        dxgi::DXGI_FORMAT_R8G8_SNORM => "R8G8_SNORM",
        dxgi::DXGI_FORMAT_R8G8_SINT => "R8G8_SINT",
        dxgi::DXGI_FORMAT_R16_TYPELESS => "R16_TYPELESS",
        dxgi::DXGI_FORMAT_R16_FLOAT => "R16_FLOAT",
        dxgi::DXGI_FORMAT_D16_UNORM => "D16_UNORM",
        dxgi::DXGI_FORMAT_R16_UNORM => "R16_UNORM",
        dxgi::DXGI_FORMAT_R16_UINT => "R16_UINT",
        dxgi::DXGI_FORMAT_R16_SNORM => "R16_SNORM",
        dxgi::DXGI_FORMAT_R16_SINT => "R16_SINT",
        dxgi::DXGI_FORMAT_R8_TYPELESS => "R8_TYPELESS",
        dxgi::DXGI_FORMAT_R8_UNORM => "R8_UNORM",
        dxgi::DXGI_FORMAT_R8_UINT => "R8_UINT",
        dxgi::DXGI_FORMAT_R8_SNORM => "R8_SNORM",
        dxgi::DXGI_FORMAT_R8_SINT => "R8_SINT",
        dxgi::DXGI_FORMAT_A8_UNORM => "A8_UNORM",
        dxgi::DXGI_FORMAT_R1_UNORM => "R1_UNORM",
        dxgi::DXGI_FORMAT_R9G9B9E5_SHAREDEXP => "R9G9B9E5_SHAREDEXP",
        dxgi::DXGI_FORMAT_R8G8_B8G8_UNORM => "R8G8_B8G8_UNORM",
        dxgi::DXGI_FORMAT_G8R8_G8B8_UNORM => "G8R8_G8B8_UNORM",
        dxgi::DXGI_FORMAT_BC1_TYPELESS => "BC1_TYPELESS",
        dxgi::DXGI_FORMAT_BC1_UNORM => "BC1_UNORM",
        dxgi::DXGI_FORMAT_BC1_UNORM_SRGB => "BC1_UNORM_SRGB",
        dxgi::DXGI_FORMAT_BC2_TYPELESS => "BC2_TYPELESS",
        dxgi::DXGI_FORMAT_BC2_UNORM => "BC2_UNORM",
        dxgi::DXGI_FORMAT_BC2_UNORM_SRGB => "BC2_UNORM_SRGB",
        dxgi::DXGI_FORMAT_BC3_TYPELESS => "BC3_TYPELESS",
        dxgi::DXGI_FORMAT_BC3_UNORM => "BC3_UNORM",
        dxgi::DXGI_FORMAT_BC3_UNORM_SRGB => "BC3_UNORM_SRGB",
        dxgi::DXGI_FORMAT_BC4_TYPELESS => "BC4_TYPELESS",
        dxgi::DXGI_FORMAT_BC4_UNORM => "BC4_UNORM",
        dxgi::DXGI_FORMAT_BC4_SNORM => "BC4_SNORM",
        dxgi::DXGI_FORMAT_BC5_TYPELESS => "BC5_TYPELESS",
        dxgi::DXGI_FORMAT_BC5_UNORM => "BC5_UNORM",
        dxgi::DXGI_FORMAT_BC5_SNORM => "BC5_SNORM",
        dxgi::DXGI_FORMAT_B5G6R5_UNORM => "B5G6R5_UNORM",
        dxgi::DXGI_FORMAT_B5G5R5A1_UNORM => "B5G5R5A1_UNORM",
        dxgi::DXGI_FORMAT_B8G8R8A8_UNORM => "B8G8R8A8_UNORM",
        dxgi::DXGI_FORMAT_B8G8R8X8_UNORM => "B8G8R8X8_UNORM",
        dxgi::DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM => "R10G10B10_XR_BIAS_A2_UNORM",
        dxgi::DXGI_FORMAT_B8G8R8A8_TYPELESS => "B8G8R8A8_TYPELESS",
        dxgi::DXGI_FORMAT_B8G8R8A8_UNORM_SRGB => "B8G8R8A8_UNORM_SRGB",
        dxgi::DXGI_FORMAT_B8G8R8X8_TYPELESS => "B8G8R8X8_TYPELESS",
        dxgi::DXGI_FORMAT_B8G8R8X8_UNORM_SRGB => "B8G8R8X8_UNORM_SRGB",
        dxgi::DXGI_FORMAT_BC6H_TYPELESS => "BC6H_TYPELESS",
        dxgi::DXGI_FORMAT_BC6H_UF16 => "BC6H_UF16",
        dxgi::DXGI_FORMAT_BC6H_SF16 => "BC6H_SF16",
        dxgi::DXGI_FORMAT_BC7_TYPELESS => "BC7_TYPELESS",
        dxgi::DXGI_FORMAT_BC7_UNORM => "BC7_UNORM",
        dxgi::DXGI_FORMAT_BC7_UNORM_SRGB => "BC7_UNORM_SRGB",
        dxgi::DXGI_FORMAT_AYUV => "AYUV",
        dxgi::DXGI_FORMAT_Y410 => "Y410",
        dxgi::DXGI_FORMAT_Y416 => "Y416",
        dxgi::DXGI_FORMAT_NV12 => "NV12",
        dxgi::DXGI_FORMAT_P010 => "P010",
        dxgi::DXGI_FORMAT_P016 => "P016",
        dxgi::DXGI_FORMAT_420_OPAQUE => "420_OPAQUE",
        dxgi::DXGI_FORMAT_YUY2 => "YUY2",
        dxgi::DXGI_FORMAT_Y210 => "Y210",
        dxgi::DXGI_FORMAT_Y216 => "Y216",
        dxgi::DXGI_FORMAT_NV11 => "NV11",
        dxgi::DXGI_FORMAT_AI44 => "AI44",
        dxgi::DXGI_FORMAT_IA44 => "IA44",
        dxgi::DXGI_FORMAT_P8 => "P8",
        dxgi::DXGI_FORMAT_A8P8 => "A8P8",
        dxgi::DXGI_FORMAT_B4G4R4A4_UNORM => "B4G4R4A4_UNORM",
        dxgi::DXGI_FORMAT_P208 => "P208",
        dxgi::DXGI_FORMAT_V208 => "V208",
        dxgi::DXGI_FORMAT_V408 => "V408",
        dxgi::DXGI_FORMAT_SAMPLER_FEEDBACK_MIN_MIP_OPAQUE => "SAMPLER_FEEDBACK_MIN_MIP_OPAQUE",
        dxgi::DXGI_FORMAT_SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE => {
            "SAMPLER_FEEDBACK_MIP_REGION_USED_OPAQUE"
        }
        dxgi::DXGI_FORMAT_FORCE_UINT => "FORCE_UINT",
        dxgi::DXGI_FORMAT_UNKNOWN | _ => "UNKNOWN",
    }
}

impl Debugger {
    pub fn new() -> anyhow::Result<Debugger> {
        use crate::module::GAME_MODULE;

        let command_stream = Mutex::new(CommandStream::new());
        let inspected_textures = HashMap::new();

        let module = unsafe {
            GAME_MODULE
                .get()
                .ok_or(anyhow::Error::msg("Failed to retrieve game module"))?
        };

        let mystery_function: fn() -> *const u8 = unsafe {
            std::mem::transmute(module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 58 60")?)
        };
        let some_global_struct = mystery_function();

        Ok(Debugger {
            command_stream,
            inspected_textures,
            some_global_struct,
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
                format: desc.Format,
            },
        );
    }

    fn draw_inspected_texture(&self, tex: &InspectedTexture) -> anyhow::Result<bool> {
        use cimgui as ig;
        use windows::Abi;

        let mut open = true;

        let base_width = tex.width as f32 / 2.0;
        let inverse_aspect_ratio = tex.height as f32 / tex.width as f32;
        ig::set_next_window_size(
            ig::Vec2::new(base_width, base_width * inverse_aspect_ratio + 40.0),
            Some(ig::Cond::Once),
        );
        ig::set_next_window_bg_alpha(1.0);
        if ig::begin(
            &format!(
                "Texture {:X?} ({}x{}, {})",
                tex.texture,
                tex.width,
                tex.height,
                dxgi_format_to_str(tex.format)
            ),
            Some(&mut open),
            None,
        )? {
            let ig::Vec2 { x: width, .. } = ig::get_window_size();
            ig::image(
                unsafe { (*tex.texture).shader_resource_view().abi() },
                ig::Vec2::new(width, width * inverse_aspect_ratio),
                None,
                None,
                None,
                None,
            );

            unsafe {
                ig::bulletf!("Texture pointer: {:X?}", (*tex.texture).texture().abi());
            }

            ig::end();
        }

        Ok(open)
    }

    fn draw_render_target(
        &mut self,
        description: &str,
        texture: *const Texture,
    ) -> anyhow::Result<()> {
        use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
        use cimgui as ig;

        let mut desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        unsafe {
            (*(*texture).texture_ptr()).GetDesc(&mut desc);
        }

        const PREVIEW_HEIGHT: f32 = 64.0;
        ig::table_next_row(None, None);
        {
            use windows::Abi;
            let aspect_ratio = desc.Width as f32 / desc.Height as f32;
            let rt_size = ig::Vec2::new(aspect_ratio * PREVIEW_HEIGHT, PREVIEW_HEIGHT);

            ig::table_next_column();
            let img = unsafe { (*(*texture).shader_resource_view_ptr()).abi() };
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
            ig::textf!("{:X?}", texture);
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
                let texture_ptr: *const Texture = unsafe {
                    let some_struct = *(self.some_global_struct.add(0x60) as *const *const u8);
                    *(some_struct.add(0x10) as *const *const Texture)
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

                    self.draw_render_target("Backbuffer", unsafe { *swapchain.back_buffer() as *const _ })?;

                    ig::end_table();
                }
            }

            if ig::collapsing_header("Render Target Manager", None, None)? {
                let textures = unsafe { render::RenderTargetManager::get().get_render_targets() };
                if ig::begin_table("xivr_debug_tab_rts_rtm", 6, None, None, None)? {
                    setup_columns(["Preview", "Address", "Offset", "Width", "Height", "Format"])?;

                    for (offset, texture) in textures.into_iter() {
                        self.draw_render_target(&format!("0x{:X}", offset), texture)?;
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
