use crate::game::graphics::kernel::{ShaderCommand, ShaderCommandType, Texture};
use crate::module::Module;

use std::collections::HashMap;
use std::string::ToString;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use strum::EnumCount;
use strum_macros::{Display, EnumCount, EnumDiscriminants};

use cimgui as ig;

struct Ptr<T>(*const T);
unsafe impl<T> Send for Ptr<T> {}
unsafe impl<T> Sync for Ptr<T> {}
impl<T> Copy for Ptr<T> {}
impl<T> Clone for Ptr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

#[derive(Display, EnumDiscriminants, EnumCount, Clone)]
enum CommandPayload {
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
    UnknownDraw,
    CopyResource,
    ResetRendererMaybe,
    Unknown1,
    CopySubresourceRegion,
    SomethingWithStrings,
    XIVRMarker(String),
}

impl CommandPayload {
    fn title(&self) -> String {
        match self {
            Self::XIVRMarker(s) => s.to_string(),
            _ => self.to_string(),
        }
    }

    fn colour(&self) -> ig::Color {
        let type_index = CommandPayloadDiscriminants::from(self) as u32;
        let hue = type_index as f32 / CommandPayload::COUNT as f32;
        ig::Color::from_hsv(hue, 0.6, 0.8)
    }
}

#[derive(Clone)]
struct Command {
    payload: CommandPayload,
    backtrace: backtrace::Backtrace,
    thread_id: u32,
    duration: Duration,
}

enum CommandStreamState {
    Uncaptured,
    WantToCapture,
    Capturing {
        start_instant: Instant,
        stream: Vec<Command>,
    },
    Captured {
        stream: Vec<Command>,
        selected_index: Option<usize>,
    },
}

pub struct CommandStream {
    state: Mutex<CommandStreamState>,
    module_name_lookup: HashMap<*mut u8, String>,
}

impl CommandStream {
    pub fn new() -> CommandStream {
        let module_name_lookup: HashMap<_, _> = Module::get_all()
            .iter()
            .map(|m| (m.base, m.filename().unwrap_or("unknown".to_string())))
            .collect();

        CommandStream {
            state: Mutex::new(CommandStreamState::Uncaptured),
            module_name_lookup,
        }
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        self.end_capture()?;
        let should_capture = if let CommandStreamState::WantToCapture = *self.state.lock().unwrap()
        {
            true
        } else {
            false
        };

        if should_capture {
            self.start_capture()?;
        }
        Ok(())
    }

    pub fn start_capture(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        *state = CommandStreamState::Capturing {
            start_instant: Instant::now(),
            stream: vec![],
        };
        Ok(())
    }

    pub fn end_capture(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        if let CommandStreamState::Capturing { stream, .. } = &*state {
            *state = CommandStreamState::Captured {
                stream: stream.clone(),
                selected_index: None,
            };
        }
        Ok(())
    }

    fn push_back_command(&mut self, payload: CommandPayload) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        if let CommandStreamState::Capturing {
            stream,
            start_instant,
        } = &mut *state
        {
            use bindings::Windows::Win32::System::Threading::GetCurrentThreadId;
            let backtrace = backtrace::Backtrace::new_unresolved();

            stream.push(Command {
                payload,
                backtrace,
                thread_id: unsafe { GetCurrentThreadId() },
                duration: Instant::now() - *start_instant,
            });
        }

        Ok(())
    }

    pub fn add_command(&mut self, cmd: &'static ShaderCommand) -> anyhow::Result<()> {
        self.push_back_command(match cmd.cmd_type {
            ShaderCommandType::SetRenderTargets => unsafe {
                let rts = cmd.payload.set_render_targets.get_render_target_slice();
                CommandPayload::SetRenderTargets(rts.iter().map(|x| Ptr(*x)).collect())
            },
            ShaderCommandType::SetViewports => CommandPayload::SetViewports,
            ShaderCommandType::SetViewportsFancy => CommandPayload::SetViewportsFancy,
            ShaderCommandType::SetScissorRect => CommandPayload::SetScissorRect,
            ShaderCommandType::Clear => CommandPayload::Clear,
            ShaderCommandType::Draw => CommandPayload::Draw,
            ShaderCommandType::DrawIndexed => CommandPayload::DrawIndexed,
            ShaderCommandType::DrawIndexedInstanced => CommandPayload::DrawIndexedInstanced,
            ShaderCommandType::DispatchComputeShader => CommandPayload::DispatchComputeShader,
            ShaderCommandType::XIVRHijack => CommandPayload::XIVRHijack,
            ShaderCommandType::CopyTexture => unsafe {
                let p = &cmd.payload.copy_texture;
                CommandPayload::CopyTexture {
                    dst: Ptr(*p.dst_resource_ptr()),
                    src: Ptr(*p.src_resource_ptr()),
                }
            },
            ShaderCommandType::UnknownDraw => CommandPayload::UnknownDraw,
            ShaderCommandType::CopyResource => CommandPayload::CopyResource,
            ShaderCommandType::ResetRendererMaybe => CommandPayload::ResetRendererMaybe,
            ShaderCommandType::Unknown1 => CommandPayload::Unknown1,
            ShaderCommandType::CopySubresourceRegion => CommandPayload::CopySubresourceRegion,
            ShaderCommandType::SomethingWithStrings => CommandPayload::SomethingWithStrings,
        })
    }

    pub fn add_marker(&mut self, msg: &str) -> anyhow::Result<()> {
        self.push_back_command(CommandPayload::XIVRMarker(msg.to_string()))
    }

    fn module_name_from_mba(&self, mba: *const u8) -> String {
        self.module_name_lookup
            .get(&(mba as *mut _))
            .cloned()
            .unwrap_or(format!("{:X?}", mba))
    }

    fn draw_cmd(&self, index: usize, cmd: &Command) -> anyhow::Result<()> {
        ig::begin_group();
        if ig::begin_child("Item view", Some(ig::Vec2::new(0.0, -1.0)), None, None)? {
            {
                ig::same_line(Some(0.0), Some(0.0));
                ig::textf!("#{}", index);

                ig::same_line(Some(0.0), Some(4.0));
                ig::push_style_color(ig::Col::Text, cmd.payload.colour());
                ig::textf!("{}", cmd.payload.title());
                ig::pop_style_color(1);
            }

            ig::separator();
            {
                ig::textf!("Thread ID: {}", cmd.thread_id);
                ig::textf!("Timestamp: {:.3}ms", cmd.duration.as_secs_f64() * 1_000.0);
            }

            ig::separator();
            if ig::collapsing_header("Data", None, None)? {
                match &cmd.payload {
                    CommandPayload::SetRenderTargets(rts) => {
                        ig::text("Render Targets: ");

                        for rt in rts {
                            ig::bullet();
                            ig::textf!("{:X?}", rt.0);
                        }
                    }
                    CommandPayload::CopyTexture { dst, src } => {
                        ig::textf!("Destination: {:X?}", dst.0);
                        ig::textf!("Source: {:X?}", src.0);
                    }
                    _ => {
                        ig::text("No additional data available.");
                    }
                }
            }

            if ig::collapsing_header("Callstack", None, None)? {
                if ig::begin_table("xivr_debugger_callstack", 2, None, None, None)? {
                    ig::table_setup_column("Module", None, None, None)?;
                    ig::table_setup_column("Address", None, None, None)?;
                    ig::table_headers_row();

                    for frame in cmd.backtrace.frames().iter().skip(10) {
                        let mba = frame.module_base_address().unwrap_or(std::ptr::null_mut());
                        let address = unsafe { frame.ip().offset_from(mba) };

                        ig::table_next_row(None, None);
                        {
                            ig::table_next_column();
                            ig::text(&self.module_name_from_mba(mba as *const _));
                            ig::table_next_column();
                            ig::textf!("0x{:0width$X}", address, width = 6);
                        }
                    }

                    ig::end_table();
                }
            }

            ig::end_child();
        }
        ig::end_group();

        Ok(())
    }

    pub fn draw_ui(&mut self) -> anyhow::Result<()> {
        let mut state = self.state.lock().unwrap();
        ig::new_line();
        {
            ig::same_line(None, Some(0.0));
            if ig::button("Capture", None)? {
                *state = CommandStreamState::WantToCapture;
            }

            ig::same_line(None, None);
            if let CommandStreamState::Captured { stream, .. } = &*state {
                ig::textf!("{} commands", stream.len());
            }
        }

        if let CommandStreamState::Captured {
            stream,
            selected_index,
        } = &mut *state
        {
            if ig::begin_child(
                "Command Stream",
                Some(ig::Vec2::new(300.0, 0.0)),
                Some(true),
                None,
            )? {
                for (i, cmd) in stream.iter().enumerate() {
                    let is_selected = *selected_index == Some(i);
                    let name = format!("{}: {}", i, cmd.payload.title());

                    ig::push_style_color(ig::Col::Text, cmd.payload.colour());
                    if ig::selectable(&name, Some(is_selected), None, None)? {
                        *selected_index = Some(i);
                    }
                    ig::pop_style_color(1);

                    if is_selected {
                        ig::set_item_default_focus();
                    }
                }
                ig::end_child();
            }

            ig::same_line(None, None);
            if let Some(index) = selected_index {
                let cmd = &stream[*index];
                self.draw_cmd(*index, cmd)?;
            }
        } else {
            ig::new_line();
            ig::separator();
            ig::text("Capture a frame to proceed.");
        }
        Ok(())
    }
}
