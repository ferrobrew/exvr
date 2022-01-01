use windows::Win32::Graphics::Direct3D11 as d3d;
use windows::Win32::Graphics::Dxgi as dxgi;

use crate::game::graphics::kernel;

pub(crate) struct Swapchain {
    swapchain: openxr::Swapchain<openxr::D3D11>,
    texture: d3d::ID3D11Texture2D,
}
impl Swapchain {
    pub(crate) fn new(
        session: &openxr::Session<openxr::D3D11>,
        size: (u32, u32),
    ) -> anyhow::Result<Swapchain> {
        use windows::runtime::Abi;

        let swapchain = session.create_swapchain(&openxr::SwapchainCreateInfo {
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

        let texture: d3d::ID3D11Texture2D = swapchain
            .enumerate_images()?
            .iter()
            .map(|x| unsafe { d3d::ID3D11Texture2D::from_abi(*x as *mut _) })
            .next()
            .ok_or_else(|| anyhow::Error::msg("Could not retrieve swapchain image!"))??;

        Ok(Swapchain { swapchain, texture })
    }

    fn acquire_image(&mut self) -> anyhow::Result<()> {
        self.swapchain.acquire_image()?;
        Ok(self.swapchain.wait_image(openxr::Duration::INFINITE)?)
    }

    fn release_image(&mut self) -> anyhow::Result<()> {
        Ok(self.swapchain.release_image()?)
    }

    pub fn copy_from_texture(&mut self, texture: d3d::ID3D11Texture2D) -> anyhow::Result<()> {
        self.acquire_image()?;
        unsafe {
            let device_context = kernel::Device::get().device_context_ptr();
            (*device_context).CopyResource(self.texture.clone(), texture);
        }
        self.release_image()
    }

    pub fn openxr_swapchain(&self) -> &openxr::Swapchain<openxr::D3D11> {
        &self.swapchain
    }
}
