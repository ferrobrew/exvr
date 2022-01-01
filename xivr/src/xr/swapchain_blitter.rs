use super::swapchain;
use crate::ct_config;
use crate::game::graphics::kernel;

use windows::Win32::Graphics::Direct3D11 as d3d;
use windows::Win32::Graphics::Dxgi as dxgi;

const SCREEN_DRAW_VERTEX_DXBC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/xivr_screen_draw_vertex.dxbc"));
const SCREEN_DRAW_PIXEL_DXBC: &[u8] = if ct_config::xr::USE_RG_DEBUG_SHADER {
    include_bytes!(concat!(
        env!("OUT_DIR"),
        "/xivr_screen_draw_rg_debug_pixel.dxbc"
    ))
} else {
    include_bytes!(concat!(env!("OUT_DIR"), "/xivr_screen_draw_pixel.dxbc"))
};

#[allow(dead_code)]
#[repr(C)]
struct Vertex {
    position: [f32; 4],
    uv: [f32; 2],
}
impl Vertex {
    const fn new(position: [f32; 4], uv: [f32; 2]) -> Vertex {
        Vertex { position, uv }
    }
}

#[allow(dead_code)]
#[repr(C)]
struct BlitParameters {
    pub total_view_count: u32,
    pub view_index: u32,
    _pad: u64,
}
impl BlitParameters {
    const fn new(view_index: u32) -> BlitParameters {
        // We're only blitting one eye to one image
        BlitParameters {
            total_view_count: 1,
            view_index,
            _pad: 0,
        }
    }
}

pub(crate) struct SwapchainBlitter {
    screen_draw_vertex: d3d::ID3D11VertexShader,
    screen_draw_pixel: d3d::ID3D11PixelShader,
    screen_draw_blit_parameters: d3d::ID3D11Buffer,
    input_layout: d3d::ID3D11InputLayout,
    vertex_buffer: d3d::ID3D11Buffer,
    sampler_state: d3d::ID3D11SamplerState,
    blend_state: d3d::ID3D11BlendState,
    rasterizer_state: d3d::ID3D11RasterizerState,
    depth_stencil_state: d3d::ID3D11DepthStencilState,
    some_global_struct: *const u8,
}
impl SwapchainBlitter {
    pub fn new(device: d3d::ID3D11Device) -> anyhow::Result<SwapchainBlitter> {
        let (screen_draw_vertex, screen_draw_pixel) = unsafe {
            use core::ffi::c_void;
            (
                device.CreateVertexShader(
                    SCREEN_DRAW_VERTEX_DXBC.as_ptr() as *const c_void,
                    SCREEN_DRAW_VERTEX_DXBC.len(),
                    None,
                )?,
                device.CreatePixelShader(
                    SCREEN_DRAW_PIXEL_DXBC.as_ptr() as *const c_void,
                    SCREEN_DRAW_PIXEL_DXBC.len(),
                    None,
                )?,
            )
        };

        let screen_draw_blit_parameters = unsafe {
            let mut default = BlitParameters::new(0);
            device.CreateBuffer(
                &d3d::D3D11_BUFFER_DESC {
                    ByteWidth: std::mem::size_of::<BlitParameters>() as u32,
                    Usage: d3d::D3D11_USAGE_DYNAMIC,
                    BindFlags: d3d::D3D11_BIND_CONSTANT_BUFFER.0,
                    CPUAccessFlags: d3d::D3D11_CPU_ACCESS_WRITE.0,
                    MiscFlags: 0,
                    StructureByteStride: 0,
                },
                &d3d::D3D11_SUBRESOURCE_DATA {
                    pSysMem: &mut default as *mut BlitParameters as *mut _,
                    SysMemPitch: 0,
                    SysMemSlicePitch: 0,
                },
            )?
        };

        let input_layout = unsafe {
            use core::ffi::c_void;

            let input_layout_definition: [d3d::D3D11_INPUT_ELEMENT_DESC; 2] = [
                d3d::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: std::mem::transmute(b"POSITION\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: dxgi::DXGI_FORMAT_R32G32B32A32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: 0,
                    InputSlotClass: d3d::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
                d3d::D3D11_INPUT_ELEMENT_DESC {
                    SemanticName: std::mem::transmute(b"UV\0".as_ptr()),
                    SemanticIndex: 0,
                    Format: dxgi::DXGI_FORMAT_R32G32_FLOAT,
                    InputSlot: 0,
                    AlignedByteOffset: d3d::D3D11_APPEND_ALIGNED_ELEMENT,
                    InputSlotClass: d3d::D3D11_INPUT_PER_VERTEX_DATA,
                    InstanceDataStepRate: 0,
                },
            ];

            device.CreateInputLayout(
                input_layout_definition.as_ptr(),
                input_layout_definition.len() as u32,
                SCREEN_DRAW_VERTEX_DXBC.as_ptr() as *const c_void,
                SCREEN_DRAW_VERTEX_DXBC.len() as usize,
            )?
        };

        let vertex_buffer = unsafe {
            const MIN: f32 = -1.0;
            const MAX: f32 = 1.0;

            const VERTICES: [Vertex; 6] = [
                Vertex::new([MAX, MAX, 0.0, 1.0], [1.0, 0.0]),
                Vertex::new([MIN, MAX, 0.0, 1.0], [0.0, 0.0]),
                Vertex::new([MIN, MIN, 0.0, 1.0], [0.0, 1.0]),
                // ---
                Vertex::new([MAX, MIN, 0.0, 1.0], [1.0, 1.0]),
                Vertex::new([MAX, MAX, 0.0, 1.0], [1.0, 0.0]),
                Vertex::new([MIN, MIN, 0.0, 1.0], [0.0, 1.0]),
            ];

            let vertex_buffer_desc = d3d::D3D11_BUFFER_DESC {
                Usage: d3d::D3D11_USAGE_DEFAULT,
                ByteWidth: (std::mem::size_of::<Vertex>() * VERTICES.len()) as u32,
                BindFlags: d3d::D3D11_BIND_VERTEX_BUFFER.0,
                CPUAccessFlags: 0,
                MiscFlags: 0,
                StructureByteStride: 0,
            };

            let vertex_data_desc = d3d::D3D11_SUBRESOURCE_DATA {
                pSysMem: VERTICES.as_ptr() as *const _ as *mut _,
                SysMemPitch: 0,
                SysMemSlicePitch: 0,
            };

            device.CreateBuffer(&vertex_buffer_desc, &vertex_data_desc)?
        };

        let sampler_state = unsafe {
            device.CreateSamplerState(&d3d::D3D11_SAMPLER_DESC {
                Filter: d3d::D3D11_FILTER_MIN_MAG_MIP_LINEAR,
                AddressU: d3d::D3D11_TEXTURE_ADDRESS_WRAP,
                AddressV: d3d::D3D11_TEXTURE_ADDRESS_WRAP,
                AddressW: d3d::D3D11_TEXTURE_ADDRESS_WRAP,
                MipLODBias: 0.0,
                MaxAnisotropy: 1,
                ComparisonFunc: d3d::D3D11_COMPARISON_ALWAYS,
                BorderColor: [0.0, 0.0, 0.0, 0.0],
                MinLOD: 0.0,
                MaxLOD: d3d::D3D11_FLOAT32_MAX,
            })?
        };

        let blend_state = unsafe {
            device.CreateBlendState(&d3d::D3D11_BLEND_DESC {
                AlphaToCoverageEnable: false.into(),
                IndependentBlendEnable: false.into(),
                RenderTarget: [d3d::D3D11_RENDER_TARGET_BLEND_DESC {
                    BlendEnable: true.into(),
                    SrcBlend: d3d::D3D11_BLEND_SRC_ALPHA,
                    DestBlend: d3d::D3D11_BLEND_INV_SRC_ALPHA,
                    BlendOp: d3d::D3D11_BLEND_OP_ADD,
                    SrcBlendAlpha: d3d::D3D11_BLEND_ONE,
                    DestBlendAlpha: d3d::D3D11_BLEND_INV_SRC_ALPHA,
                    BlendOpAlpha: d3d::D3D11_BLEND_OP_ADD,
                    RenderTargetWriteMask: d3d::D3D11_COLOR_WRITE_ENABLE_ALL.0 as u8,
                }; 8],
            })?
        };

        let rasterizer_state = unsafe {
            device.CreateRasterizerState(&d3d::D3D11_RASTERIZER_DESC {
                FillMode: d3d::D3D11_FILL_SOLID,
                CullMode: d3d::D3D11_CULL_NONE,
                FrontCounterClockwise: false.into(),
                DepthBias: 0,
                DepthBiasClamp: 0.0,
                SlopeScaledDepthBias: 0.0,
                ScissorEnable: true.into(),
                DepthClipEnable: true.into(),
                MultisampleEnable: false.into(),
                AntialiasedLineEnable: false.into(),
            })?
        };

        let depth_stencil_state = unsafe {
            device.CreateDepthStencilState(&d3d::D3D11_DEPTH_STENCIL_DESC {
                DepthEnable: false.into(),
                DepthWriteMask: d3d::D3D11_DEPTH_WRITE_MASK_ALL,
                DepthFunc: d3d::D3D11_COMPARISON_ALWAYS,
                StencilEnable: false.into(),
                StencilReadMask: 0,
                StencilWriteMask: 0,
                FrontFace: d3d::D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilFunc: d3d::D3D11_COMPARISON_ALWAYS,
                },
                BackFace: d3d::D3D11_DEPTH_STENCILOP_DESC {
                    StencilFailOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilDepthFailOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilPassOp: d3d::D3D11_STENCIL_OP_KEEP,
                    StencilFunc: d3d::D3D11_COMPARISON_ALWAYS,
                },
            })?
        };

        let some_global_struct = unsafe {
            let module = crate::util::game_module_mut()?;
            let mystery_function: fn() -> *const u8 =
                std::mem::transmute(module.scan_for_relative_callsite("E8 ? ? ? ? 48 8B 58 60")?);
            mystery_function()
        };

        Ok(SwapchainBlitter {
            screen_draw_vertex,
            screen_draw_pixel,
            screen_draw_blit_parameters,
            input_layout,
            vertex_buffer,
            sampler_state,
            blend_state,
            rasterizer_state,
            depth_stencil_state,
            some_global_struct,
        })
    }

    pub unsafe fn blit_to_buffer(
        &mut self,
        swapchain: &swapchain::Swapchain,
        index: u32,
    ) -> anyhow::Result<()> {
        let dc = kernel::Device::get().device_context();

        // Before we do any rendering, update our constant buffer to have the correct data.
        let mapped_resource = dc.Map(
            self.screen_draw_blit_parameters.clone(),
            0,
            d3d::D3D11_MAP_WRITE_DISCARD,
            0,
        )?;
        *(mapped_resource.pData as *mut BlitParameters) = BlitParameters::new(index);
        dc.Unmap(self.screen_draw_blit_parameters.clone(), 0);

        let rtv = Some(swapchain.buffer_rtv.clone());
        dc.ClearRenderTargetView(rtv.clone(), [1.0, 0.0, 0.0, 0.0].as_ptr());

        let vertex_count = std::mem::size_of::<Vertex>() as u32;
        let offset = 0;
        let vb = Some(self.vertex_buffer.clone());
        dc.IASetVertexBuffers(0, 1, &vb, &vertex_count, &offset);
        dc.IASetPrimitiveTopology(d3d::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        dc.IASetInputLayout(&self.input_layout);

        let cb = Some(self.screen_draw_blit_parameters.clone());
        dc.VSSetConstantBuffers(0, 1, &cb);
        dc.VSSetShader(&self.screen_draw_vertex, std::ptr::null_mut(), 0);

        dc.PSSetShader(&self.screen_draw_pixel, std::ptr::null_mut(), 0);
        {
            let texture: &kernel::Texture = {
                let some_struct = *(self.some_global_struct.add(0x60) as *const *const u8);
                &**(some_struct.add(0x10) as *const *const kernel::Texture)
            };
            let srv = texture.shader_resource_view().clone().map(|x| x.into());
            dc.PSSetShaderResources(0, 1, &srv);
        }
        {
            let sampler_state = Some(self.sampler_state.clone());
            dc.PSSetSamplers(0, 1, &sampler_state);
        }

        dc.OMSetBlendState(
            self.blend_state.clone(),
            [0.0, 0.0, 0.0, 0.0].as_ptr(),
            0xFFFF_FFFF,
        );
        dc.OMSetDepthStencilState(self.depth_stencil_state.clone(), 0);
        dc.OMSetRenderTargets(1, &rtv, None);

        dc.RSSetState(self.rasterizer_state.clone());
        dc.RSSetViewports(
            1,
            &d3d::D3D11_VIEWPORT {
                Width: swapchain.frame_size.0 as f32,
                Height: swapchain.frame_size.1 as f32,
                MinDepth: 0.0,
                MaxDepth: 1.0,
                TopLeftX: 0.0,
                TopLeftY: 0.0,
            },
        );

        dc.Draw(6, 0);

        Ok(())
    }
}
