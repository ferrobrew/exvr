use windows::Win32::Graphics::Direct3D11 as d3d;
use windows::Win32::Graphics::Dxgi as dxgi;

use crate::game::graphics::kernel;

pub(crate) struct Swapchain {
    swapchain: openxr::Swapchain<openxr::D3D11>,
    swapchain_image: d3d::ID3D11Texture2D,
    buffer_image: d3d::ID3D11Texture2D,
    buffer_srv: d3d::ID3D11ShaderResourceView,
    pub buffer_rtv: d3d::ID3D11RenderTargetView,
    pub size: (u32, u32),
}
impl Swapchain {
    pub(crate) fn new(
        session: &openxr::Session<openxr::D3D11>,
        device: d3d::ID3D11Device,
        size: (u32, u32),
    ) -> anyhow::Result<Swapchain> {
        use windows::runtime::Abi;

        let mut swapchain = session.create_swapchain(&openxr::SwapchainCreateInfo {
            create_flags: openxr::SwapchainCreateFlags::EMPTY,
            usage_flags: openxr::SwapchainUsageFlags::COLOR_ATTACHMENT
                | openxr::SwapchainUsageFlags::SAMPLED,
            format: dxgi::DXGI_FORMAT_R8G8B8A8_UNORM.0,
            sample_count: 1,
            width: size.0,
            height: size.1,
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
            size,
        })
    }

    fn acquire_image(&mut self) -> anyhow::Result<()> {
        self.swapchain.acquire_image()?;
        Ok(self.swapchain.wait_image(openxr::Duration::INFINITE)?)
    }

    fn release_image(&mut self) -> anyhow::Result<()> {
        Ok(self.swapchain.release_image()?)
    }

    pub fn copy_from_buffer(&mut self) -> anyhow::Result<()> {
        self.acquire_image()?;
        unsafe {
            let device_context = kernel::Device::get().device_context_ptr();
            (*device_context).CopyResource(self.swapchain_image.clone(), self.buffer_image.clone());
        }
        self.release_image()
    }

    pub fn render_button(&self, size: cimgui::Vec2, color: cimgui::Color) -> anyhow::Result<()> {
        if cimgui::image_button(
            unsafe { std::mem::transmute(self.buffer_srv.clone()) },
            size,
            None,
            None,
            None,
            Some(color),
            None,
        ) {
            use crate::debugger::Debugger;

            if let Some(debugger) = Debugger::get_mut() {
                debugger.inspect_d3d_texture(
                    self.buffer_image.clone(),
                    Some(self.buffer_srv.clone()),
                )?;
            }
        }

        Ok(())
    }

    pub fn openxr_swapchain(&self) -> &openxr::Swapchain<openxr::D3D11> {
        &self.swapchain
    }
}
