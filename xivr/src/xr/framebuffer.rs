use windows::Win32::Graphics::Direct3D11 as d3d;
use windows::Win32::Graphics::Dxgi as dxgi;

pub(crate) struct Framebuffer {
    texture: d3d::ID3D11Texture2D,
    srv: d3d::ID3D11ShaderResourceView,
    rtv: d3d::ID3D11RenderTargetView,
    size: (u32, u32),
}

impl Framebuffer {
    pub(crate) fn new(device: d3d::ID3D11Device, size: (u32, u32)) -> anyhow::Result<Framebuffer> {
        let texture_format = dxgi::DXGI_FORMAT_R8G8B8A8_UNORM;

        let texture: d3d::ID3D11Texture2D = unsafe {
            device.CreateTexture2D(
                &d3d::D3D11_TEXTURE2D_DESC {
                    Width: size.0,
                    Height: size.1,
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

        let srv = unsafe {
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
            device.CreateShaderResourceView(texture.clone(), &desc)?
        };

        let rtv = unsafe {
            let desc = d3d::D3D11_RENDER_TARGET_VIEW_DESC {
                Format: texture_format,
                ViewDimension: d3d::D3D11_RTV_DIMENSION_TEXTURE2D,
                Anonymous: d3d::D3D11_RENDER_TARGET_VIEW_DESC_0 {
                    Texture2D: d3d::D3D11_TEX2D_RTV { MipSlice: 0 },
                },
            };
            device.CreateRenderTargetView(texture.clone(), &desc)?
        };

        Ok(Framebuffer {
            texture,
            srv,
            rtv,
            size,
        })
    }

    pub fn render_button(&self, size: cimgui::Vec2, color: cimgui::Color) -> anyhow::Result<()> {
        if cimgui::image_button(
            unsafe { std::mem::transmute(self.srv.clone()) },
            size,
            None,
            None,
            None,
            Some(color),
            None,
        ) {
            use crate::debugger::Debugger;

            if let Some(debugger) = Debugger::get_mut() {
                debugger.inspect_d3d_texture(self.texture.clone(), Some(self.srv.clone()))?;
            }
        }

        Ok(())
    }

    pub fn texture(&self) -> d3d::ID3D11Texture2D {
        self.texture.clone()
    }

    pub fn rtv(&self) -> d3d::ID3D11RenderTargetView {
        self.rtv.clone()
    }

    pub fn size(&self) -> (u32, u32) {
        self.size
    }
}
