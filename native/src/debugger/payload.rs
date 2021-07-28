use crate::game::graphics::kernel;

use std::time::Duration;

pub trait Payload {
    fn title(&self) -> String;
    fn colour(&self) -> cimgui::Color;
    fn draw(&self) -> anyhow::Result<()>;
}

#[derive(Clone)]
pub struct Command<PayloadType> {
    pub payload: PayloadType,
    pub address: Option<*const kernel::ShaderCommand>,
    pub backtrace: backtrace::Backtrace,
    pub thread_id: u32,
    pub duration: Duration,
}
