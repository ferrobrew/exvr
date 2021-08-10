use crate::debugger::payload::*;

use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants};

use crate::game::graphics::kernel::Texture;

use cimgui as ig;

pub struct Ptr<T>(pub *const T);
unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}
impl<T> Copy for Ptr<T> {}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
pub enum ShaderPayload {
    SetRenderTargets(Vec<Ptr<Texture>>),
    SetViewports,
    SetViewportsFancy,
    SetScissorRect,
    Clear,
    Draw,
    DrawIndexed,
    DrawIndexedInstanced,
    DispatchComputeShader,
    XIVRHijack,
    CopyTexture {
        dst: Ptr<Texture>,
        src: Ptr<Texture>,
    },
    UnknownDraw {
        render_target: Ptr<Texture>,
        sampled_texture: Ptr<Texture>,
    },
    CopyResource,
    ResetRendererMaybe,
    Unknown1,
    CopySubresourceRegion,
    SomethingWithStrings,
    XIVRMarker(String),
}

impl Payload for ShaderPayload {
    fn title(&self) -> String {
        match self {
            Self::XIVRMarker(s) => s.to_string(),
            _ => self.to_string(),
        }
    }

    fn colour(&self) -> ig::Color {
        let type_index = ShaderPayloadDiscriminants::from(self) as u32;
        let hue = type_index as f32 / ShaderPayload::COUNT as f32;
        ig::Color::from_hsv(hue, 0.6, 0.8)
    }

    fn draw(&self) -> anyhow::Result<()> {
        use crate::debugger::Debugger;
        if let Some(debugger) = Debugger::get_mut() {
            match self {
                ShaderPayload::SetRenderTargets(rts) => {
                    ig::text("Render Targets: ");

                    for rt in rts {
                        ig::bullet();
                        if ig::small_button(&format!("{:X?}", rt.0))? {
                            debugger.inspect_texture(unsafe { &*rt.0 });
                        }
                    }
                }
                ShaderPayload::CopyTexture { dst, src } => {
                    ig::text("Destination: ");
                    ig::same_line(None, Some(0.0));
                    if ig::small_button(&format!("{:X?}", dst.0))? {
                        debugger.inspect_texture(unsafe { &*dst.0 });
                    }

                    ig::text("Source: ");
                    ig::same_line(None, Some(0.0));
                    if ig::small_button(&format!("{:X?}", src.0))? {
                        debugger.inspect_texture(unsafe { &*src.0 });
                    }
                }
                ShaderPayload::UnknownDraw {
                    render_target,
                    sampled_texture,
                } => {
                    ig::text("Render Target: ");
                    ig::same_line(None, Some(0.0));
                    if ig::small_button(&format!("{:X?}", render_target.0))? {
                        debugger.inspect_texture(unsafe { &*render_target.0 });
                    }

                    ig::text("Sampled Texture: ");
                    ig::same_line(None, Some(0.0));
                    if ig::small_button(&format!("{:X?}", sampled_texture.0))? {
                        debugger.inspect_texture(unsafe { &*sampled_texture.0 });
                    }
                }
                _ => {
                    ig::text("No additional data available.");
                }
            }
        }

        Ok(())
    }
}

pub type ShaderCommand = Command<ShaderPayload>;
