use crate::ct_config;
use crate::debugger::Debugger;
use crate::game::graphics::kernel;
use crate::game::system::framework;
use crate::singleton;

use bindings::Windows::Win32::Graphics::Direct3D11 as d3d;
use bindings::Windows::Win32::Graphics::Dxgi as dxgi;
use windows::Abi;

pub use crate::ct_config::xr::VIEW_COUNT;
const VIEW_TYPE: openxr::ViewConfigurationType = openxr::ViewConfigurationType::PRIMARY_STEREO;

const SCREEN_DRAW_VERTEX_DXBC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/xivr_screen_draw_vertex.dxbc"));
const SCREEN_DRAW_PIXEL_DXBC: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/xivr_screen_draw_pixel.dxbc"));

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
    const fn new(_view_index: u32) -> BlitParameters {
        // We're only blitting one eye to one image
        BlitParameters {
            total_view_count: 1,
            view_index: 0,
            _pad: 0,
        }
    }
}

struct Swapchain {
    swapchain: openxr::Swapchain<openxr::D3D11>,
    swapchain_image: d3d::ID3D11Texture2D,
    buffer_image: d3d::ID3D11Texture2D,
    pub buffer_srv: d3d::ID3D11ShaderResourceView,
    pub buffer_rtv: d3d::ID3D11RenderTargetView,
    pub frame_size: (u32, u32),
    pub index: u32,
}
impl Swapchain {
    fn new(
        session: &openxr::Session<openxr::D3D11>,
        device: d3d::ID3D11Device,
        frame_size: (u32, u32),
        index: u32,
    ) -> anyhow::Result<Swapchain> {
        let mut swapchain = session.create_swapchain(&openxr::SwapchainCreateInfo {
            create_flags: openxr::SwapchainCreateFlags::EMPTY,
            usage_flags: openxr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | openxr::SwapchainUsageFlags::SAMPLED,
            format: dxgi::DXGI_FORMAT_R8G8B8A8_UNORM.0,
            sample_count: 1,
            width: frame_size.0,
            height: frame_size.1,
            face_count: 1,
            array_size: 1,
            mip_count: 1,
        })?;

        let swapchain_image: d3d::ID3D11Texture2D = swapchain
            .enumerate_images()?
            .iter()
            .map(|x| unsafe { d3d::ID3D11Texture2D::from_abi(*x as *mut _) })
            .next()
            .ok_or_else(|| anyhow::Error::msg("Could not retrieve swapchain image!"))??;

        let mut swapchain_desc: d3d::D3D11_TEXTURE2D_DESC = unsafe { std::mem::zeroed() };
        {
            swapchain.acquire_image()?;
            swapchain.wait_image(openxr::Duration::INFINITE)?;
            unsafe {
                swapchain_image.GetDesc(&mut swapchain_desc);
            }
            swapchain.release_image()?;
        }

        let texture_format = dxgi::DXGI_FORMAT_R8G8B8A8_UNORM;

        let buffer_image: d3d::ID3D11Texture2D = unsafe {
            device.CreateTexture2D(
                &d3d::D3D11_TEXTURE2D_DESC {
                    Width: swapchain_desc.Width,
                    Height: swapchain_desc.Height,
                    MipLevels: 1,
                    ArraySize: 1,
                    Format: texture_format,
                    SampleDesc: dxgi::DXGI_SAMPLE_DESC {
                        Count: 1,
                        Quality: 0,
                    },
                    Usage: d3d::D3D11_USAGE_DEFAULT,
                    BindFlags: d3d::D3D11_BIND_SHADER_RESOURCE | d3d::D3D11_BIND_RENDER_TARGET,
                    CPUAccessFlags: d3d::D3D11_CPU_ACCESS_FLAG(0),
                    MiscFlags: d3d::D3D11_RESOURCE_MISC_FLAG(0),
                },
                std::ptr::null(),
            )?
        };

        let buffer_srv = unsafe {
            let desc = d3d::D3D11_SHADER_RESOURCE_VIEW_DESC {
                Format: texture_format,
                ViewDimension: d3d::D3D_SRV_DIMENSION_TEXTURE2D,
                Anonymous: d3d::D3D11_SHADER_RESOURCE_VIEW_DESC_0 {
                    Texture2D: d3d::D3D11_TEX2D_SRV {
                        MostDetailedMip: 0,
                        MipLevels: 1,
                    },
                },
            };
            device.CreateShaderResourceView(buffer_image.clone(), &desc)?
        };

        let buffer_rtv = unsafe {
            let desc = d3d::D3D11_RENDER_TARGET_VIEW_DESC {
                Format: texture_format,
                ViewDimension: d3d::D3D11_RTV_DIMENSION_TEXTURE2D,
                Anonymous: d3d::D3D11_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2D: d3d::D3D11_TEX2D_RTV { MipSlice: 0 },
                },
            };
            device.CreateRenderTargetView(buffer_image.clone(), &desc)?
        };

        Ok(Swapchain {
            swapchain,
            swapchain_image,
            buffer_image,
            buffer_srv,
            buffer_rtv,
            frame_size,
            index,
        })
    }

    fn copy_from_buffer(&mut self) -> anyhow::Result<()> {
        self.swapchain.acquire_image()?;
        self.swapchain.wait_image(openxr::Duration::INFINITE)?;
        unsafe {
            let device_context = kernel::Device::get().device_context_ptr();

            (*device_context).CopyResource(self.swapchain_image.clone(), self.buffer_image.clone());
        }
        Ok(self.swapchain.release_image()?)
    }

    fn render_button(&self, size: cimgui::Vec2, color: cimgui::Color) -> anyhow::Result<()> {
        if cimgui::image_button(
            self.buffer_srv.abi(),
            size,
            None,
            None,
            None,
            Some(color),
            None,
        ) {
            if let Some(debugger) = Debugger::get_mut() {
                debugger.inspect_d3d_texture(
                    self.buffer_image.clone(),
                    Some(self.buffer_srv.clone()),
                )?;
            }
        }

        Ok(())
    }
}

struct SwapchainBlitter {
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
    fn new(device: d3d::ID3D11Device) -> anyhow::Result<SwapchainBlitter> {
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
            use crate::module::GAME_MODULE;
            let module = GAME_MODULE
                .get()
                .ok_or_else(|| anyhow::Error::msg("Failed to retrieve game module"))?;

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

    pub unsafe fn blit_to_buffer(&mut self, swapchain: &Swapchain) -> anyhow::Result<()> {
        let dc = kernel::Device::get().device_context();

        // Before we do any rendering, update our constant buffer to have the correct data.
        let mapped_resource = dc.Map(
            self.screen_draw_blit_parameters.clone(),
            0,
            d3d::D3D11_MAP_WRITE_DISCARD,
            0,
        )?;
        *(mapped_resource.pData as *mut BlitParameters) = BlitParameters::new(swapchain.index);
        dc.Unmap(self.screen_draw_blit_parameters.clone(), 0);

        let mut rtv = Some(swapchain.buffer_rtv.clone());
        dc.ClearRenderTargetView(rtv.clone(), [0.0, 0.0, 0.0, 0.0].as_ptr());

        let vertex_count = std::mem::size_of::<Vertex>() as u32;
        let offset = 0;
        let mut vb = Some(self.vertex_buffer.clone());
        dc.IASetVertexBuffers(0, 1, &mut vb, &vertex_count, &offset);
        dc.IASetPrimitiveTopology(d3d::D3D11_PRIMITIVE_TOPOLOGY_TRIANGLELIST);
        dc.IASetInputLayout(&self.input_layout);

        let mut cb = Some(self.screen_draw_blit_parameters.clone());
        dc.VSSetConstantBuffers(0, 1, &mut cb);
        dc.VSSetShader(&self.screen_draw_vertex, std::ptr::null_mut(), 0);

        dc.PSSetShader(&self.screen_draw_pixel, std::ptr::null_mut(), 0);
        {
            let texture: &kernel::Texture = {
                let some_struct = *(self.some_global_struct.add(0x60) as *const *const u8);
                &**(some_struct.add(0x10) as *const *const kernel::Texture)
            };
            let srv: d3d::ID3D11ShaderResourceView = texture.shader_resource_view().clone().into();
            let mut srv = Some(srv);
            dc.PSSetShaderResources(0, 1, &mut srv);
        }
        {
            let mut sampler_state = Some(self.sampler_state.clone());
            dc.PSSetSamplers(0, 1, &mut sampler_state);
        }

        dc.OMSetBlendState(
            self.blend_state.clone(),
            [0.0, 0.0, 0.0, 0.0].as_ptr(),
            0xFFFF_FFFF,
        );
        dc.OMSetDepthStencilState(self.depth_stencil_state.clone(), 0);
        dc.OMSetRenderTargets(1, &mut rtv, None);

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

#[allow(dead_code)]
pub struct XR {
    instance: openxr::Instance,
    session: openxr::Session<openxr::D3D11>,
    frame_waiter: openxr::FrameWaiter,
    frame_stream: openxr::FrameStream<openxr::D3D11>,
    stage: openxr::Space,
    view_configuration_views: Vec<openxr::ViewConfigurationView>,
    session_running: bool,

    swapchains: Vec<Swapchain>,
    swapchain_blitter: SwapchainBlitter,

    environment_blend_mode: openxr::EnvironmentBlendMode,
    frame_state: Option<openxr::FrameState>,
    view_state_flags: openxr::ViewStateFlags,
    views: Vec<openxr::View>,

    old_window_size: (u32, u32),
    frame_size: (u32, u32),

    instance_properties: openxr::InstanceProperties,
    system_properties: openxr::SystemProperties,
    available_extensions: openxr::ExtensionSet,
}
singleton!(XR);

impl XR {
    pub fn new() -> anyhow::Result<XR> {
        let entry = openxr::Entry::linked();
        let available_extensions = entry.enumerate_extensions()?;
        assert!(available_extensions.khr_d3d11_enable);

        let mut enabled_extensions = openxr::ExtensionSet::default();
        enabled_extensions.khr_d3d11_enable = true;
        let instance = entry.create_instance(
            &openxr::ApplicationInfo {
                application_name: "XIVR",
                application_version: 0,
                engine_name: "XIVR",
                engine_version: 0,
            },
            &enabled_extensions,
            &[],
        )?;
        let instance_properties = instance.properties()?;

        // Request a form factor from the device (HMD, Handheld, etc.)
        let system = instance.system(openxr::FormFactor::HEAD_MOUNTED_DISPLAY)?;

        // Check what blend mode is valid for this device (opaque vs transparent displays). We'll just
        // take the first one available!
        let environment_blend_mode =
            instance.enumerate_environment_blend_modes(system, VIEW_TYPE)?[0];

        // We don't do anything with reqs as running this version of the game necessitates that you have
        // support for D3D11.
        let _reqs = instance.graphics_requirements::<openxr::D3D11>(system)?;

        let system_properties = instance.system_properties(system)?;

        let view_configuration_views =
            instance.enumerate_view_configuration_views(system, VIEW_TYPE)?;
        assert_eq!(view_configuration_views[0], view_configuration_views[1]);

        let old_window_size = unsafe { framework::Framework::get().window().get_size() };
        let new_window_size = if ct_config::xr::CHANGE_WINDOW_SIZE {
            (
                view_configuration_views[0].recommended_image_rect_width,
                view_configuration_views[0].recommended_image_rect_height,
            )
        } else {
            old_window_size
        };

        let device = unsafe { kernel::Device::get().device() };
        let (session, frame_waiter, frame_stream) = unsafe {
            instance.create_session::<openxr::D3D11>(
                system,
                &openxr::d3d::SessionCreateInfo {
                    device: std::mem::transmute(device.abi()),
                },
            )?
        };

        let stage = session
            .create_reference_space(openxr::ReferenceSpaceType::STAGE, openxr::Posef::IDENTITY)?;

        unsafe {
            let window = framework::Framework::get().window_mut();
            window.set_resizing_enabled(false);
            window.set_size(new_window_size);
        }

        let swapchains = (0..VIEW_COUNT)
            .map(|index| Swapchain::new(&session, device.clone(), new_window_size, index))
            .collect::<anyhow::Result<Vec<_>>>()?;
        let swapchain_blitter = SwapchainBlitter::new(device.clone())?;

        Ok(XR {
            instance,
            session,
            frame_waiter,
            frame_stream,
            stage,
            view_configuration_views,
            session_running: false,

            swapchains,
            swapchain_blitter,

            environment_blend_mode,
            frame_state: None,
            view_state_flags: openxr::ViewStateFlags::EMPTY,
            views: vec![],

            old_window_size,
            frame_size: new_window_size,

            instance_properties,
            system_properties,
            available_extensions,
        })
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        let session = &self.session;
        let mut event_storage = openxr::EventDataBuffer::new();

        while let Some(event) = self.instance.poll_event(&mut event_storage)? {
            use openxr::Event::*;
            use openxr::SessionState;
            match event {
                SessionStateChanged(e) => {
                    // Session state change is where we can begin and end sessions, as well as
                    // find quit messages!
                    log!("entered state {:?}", e.state());
                    match e.state() {
                        SessionState::READY => {
                            session.begin(VIEW_TYPE)?;
                            self.session_running = true;
                        }
                        SessionState::STOPPING => {
                            session.end()?;
                            self.session_running = false;
                        }
                        SessionState::EXITING | SessionState::LOSS_PENDING => {
                            break;
                        }
                        _ => {}
                    }
                }
                InstanceLossPending(_) => {
                    break;
                }
                EventsLost(e) => {
                    log!("lost {} events", e.lost_event_count());
                }
                _ => {}
            }
        }

        if self.session_running && ct_config::xr::RUN_XR_PER_FRAME {
            if let Some(frame_state) = self.frame_state {
                let (view_state_flags, views) = self.session.locate_views(
                    VIEW_TYPE,
                    frame_state.predicted_display_time,
                    &self.stage,
                )?;
                self.view_state_flags = view_state_flags;
                self.views = views;
            }
        }

        Ok(())
    }

    pub fn post_update(&mut self) -> anyhow::Result<()> {
        if self.session_running && ct_config::xr::RUN_XR_PER_FRAME {
            self.frame_state = Some(self.frame_waiter.wait()?);
        }

        Ok(())
    }

    pub fn pre_render(&mut self) -> anyhow::Result<()> {
        if self.session_running && ct_config::xr::RUN_XR_PER_FRAME {
            self.frame_stream.begin()?;
        }

        Ok(())
    }

    pub fn post_render(&mut self) -> anyhow::Result<()> {
        if !(self.session_running && ct_config::xr::RUN_XR_PER_FRAME) {
            return Ok(());
        }

        if self.frame_state.is_none() {
            return Ok(());
        }

        let frame_state = self.frame_state.as_ref().unwrap();

        for swapchain in &mut self.swapchains {
            swapchain.copy_from_buffer()?;
        }

        let views = self
            .views
            .iter()
            .zip(&self.swapchains)
            .map(|(v, s)| {
                let rect = openxr::Rect2Di {
                    offset: openxr::Offset2Di { x: 0, y: 0 },
                    extent: openxr::Extent2Di {
                        width: self.frame_size.0 as _,
                        height: self.frame_size.1 as _,
                    },
                };

                openxr::CompositionLayerProjectionView::new()
                    .pose(v.pose)
                    .fov(v.fov)
                    .sub_image(
                        openxr::SwapchainSubImage::new()
                            .swapchain(&s.swapchain)
                            .image_rect(rect),
                    )
            })
            .collect::<Vec<_>>();

        self.frame_stream.end(
            frame_state.predicted_display_time,
            self.environment_blend_mode,
            &[&openxr::CompositionLayerProjection::new()
                .space(&self.stage)
                .views(&views)],
        )?;

        Ok(())
    }

    pub fn draw_ui_framebuffers(&mut self) -> anyhow::Result<()> {
        use cimgui as ig;

        let ig::Vec2 { x: width, .. } = ig::get_window_size();
        let inverse_aspect_ratio = self.frame_size.1 as f32 / self.frame_size.0 as f32;
        let srv_width = (width - 32.0) / (VIEW_COUNT as f32);
        let size = ig::Vec2::new(srv_width, srv_width * inverse_aspect_ratio);
        let color = ig::Color::new(0.0, 0.0, 0.0, 1.0);

        ig::new_line();
        for swapchain in &self.swapchains {
            ig::same_line(None, Some(0.0));
            swapchain.render_button(size, color)?;
        }

        Ok(())
    }

    #[rustfmt::skip]
    pub fn draw_ui_properties(&mut self) -> anyhow::Result<()> {
        use cimgui as ig;

        if ig::collapsing_header("Config", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            ig::bulletf!("data.yml version: {}", crate::game::VERSION);
        }

        if ig::collapsing_header("Instance Properties", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let inst_props = &self.instance_properties;
            ig::bulletf!("Runtime name: {}", inst_props.runtime_name);
            ig::bulletf!("Runtime version: {}", inst_props.runtime_version);
        }

        if ig::collapsing_header("System Properties", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let sys_props = &self.system_properties;
            ig::bulletf!("System name: {}", sys_props.system_name);
            ig::bulletf!("Vendor ID: {}", sys_props.vendor_id);
            ig::bulletf!("System ID: {:?}", sys_props.system_id);
            ig::bulletf!("Orientation Tracking: {}", sys_props.tracking_properties.orientation_tracking);
            ig::bulletf!("Position Tracking: {}", sys_props.tracking_properties.position_tracking);
        }

        if ig::collapsing_header("Extensions", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            let exts = &self.available_extensions;
            ig::bulletf!("ext_performance_settings: {}", exts.ext_performance_settings);
            ig::bulletf!("ext_debug_utils: {}", exts.ext_debug_utils);
            ig::bulletf!("ext_eye_gaze_interaction: {}", exts.ext_eye_gaze_interaction);
            ig::bulletf!("ext_hand_tracking: {}", exts.ext_hand_tracking);
            ig::bulletf!("ext_hand_joints_motion_range: {}", exts.ext_hand_joints_motion_range);
            ig::bulletf!("msft_hand_interaction: {}", exts.msft_hand_interaction);
            ig::bulletf!("msft_hand_tracking_mesh: {}", exts.msft_hand_tracking_mesh);
            ig::bulletf!("msft_controller_model: {}", exts.msft_controller_model);
        }

        if ig::collapsing_header("Frame", None, Some(ig::TreeNodeFlags::DefaultOpen))? {
            ig::bulletf!("Frame size: {}x{}", self.frame_size.0, self.frame_size.1);
            ig::bulletf!("Original window size: {}x{}", self.old_window_size.0, self.old_window_size.1);
        }

        Ok(())
    }

    pub fn copy_backbuffer_to_buffer(&mut self, index: u32) -> anyhow::Result<()> {
        unsafe {
            self.swapchain_blitter.blit_to_buffer(
                self.swapchains
                    .get(index as usize)
                    .ok_or_else(|| anyhow::anyhow!("no swapchain for index"))?,
            )
        }
    }
}

impl Drop for XR {
    fn drop(&mut self) {
        if ct_config::xr::CHANGE_WINDOW_SIZE {
            unsafe {
                let window: &mut _ = framework::Framework::get().window_mut();

                window.set_resizing_enabled(true);
                window.set_size(self.old_window_size);
            }
        }
    }
}
