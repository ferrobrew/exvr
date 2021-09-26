use crate::debugger::payload::{Command, Payload};

use cimgui as ig;

pub type MessagePayload = (String, Vec<String>);

impl Payload for MessagePayload {
    fn title(&self) -> String {
        self.0.clone()
    }

    fn colour(&self) -> ig::Color {
        ig::Color::from_hsv((self.1.len() as f32 / 128.0).min(1.0), 0.7, 0.8)
    }

    fn draw(&self) -> anyhow::Result<()> {
        for t in &self.1 {
            ig::bulletf!("{}", t);
        }
        Ok(())
    }
}

// lol this suffix is getting a bit silly now
pub type MessageCommand = Command<MessagePayload>;
