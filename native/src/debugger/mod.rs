mod command_stream;
pub use command_stream::*;

use crate::singleton;

pub struct Debugger {
    pub command_stream: CommandStream,
}
singleton!(Debugger);

impl Debugger {
    pub fn new() -> anyhow::Result<Debugger> {
        let command_stream = CommandStream::new();
        Ok(Debugger { command_stream })
    }

    pub fn draw_ui(&mut self) -> anyhow::Result<()> {
        use crate::xr::XR;
        use cimgui as ig;

        if ig::begin("XIVR Debugger", None, None)? {
            if ig::begin_tab_bar("xivr_debug_tabs", None)? {
                if ig::begin_tab_item("Command Stream", None, None)? {
                    self.command_stream.draw_ui()?;
                    ig::end_tab_item();
                }
                if let Some(xr) = XR::get_mut() {
                    if ig::begin_tab_item("Framebuffers", None, None)? {
                        xr.draw_ui_framebuffers()?;
                        ig::end_tab_item();
                    }
                    if ig::begin_tab_item("Render Targets", None, None)? {
                        xr.draw_ui_render_targets()?;
                        ig::end_tab_item();
                    }
                    ig::end_tab_bar();
                }
                ig::end();
            }
        }

        Ok(())
    }
}