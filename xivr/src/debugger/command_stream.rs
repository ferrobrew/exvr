use crate::ct_config;

use crate::debugger::d3d_payload::*;
use crate::debugger::payload::*;
use crate::debugger::shader_payload::*;
use crate::debugger::message_payload::*;

use crate::game::graphics::kernel;
use crate::game::graphics::kernel::ShaderCommandType;
use crate::module::Module;

use std::collections::HashMap;
use std::string::ToString;
use std::time::Instant;

use cimgui as ig;

const FRAMES_TO_CAPTURE: u32 = 1;

struct Stream<CommandType> {
    stream: Vec<CommandType>,
    selected_index: Option<usize>,
}

impl<CommandType> From<Vec<CommandType>> for Stream<CommandType> {
    fn from(val: Vec<CommandType>) -> Self {
        Stream {
            stream: val,
            selected_index: None,
        }
    }
}

enum CommandStreamState {
    Uncaptured,
    WantToCapture,
    Capturing {
        start_instant: Instant,
        shader_stream: Vec<ShaderCommand>,
        processed_shader_stream: Vec<ShaderCommand>,
        d3d_stream: Vec<D3DCommand>,
        message_stream: Vec<MessageCommand>,
        frames: u32,
    },
    Captured {
        shader_streams: HashMap<u32, Stream<ShaderCommand>>,
        processed_shader_stream: Stream<ShaderCommand>,
        d3d_stream: Stream<D3DCommand>,
        message_stream: Stream<MessageCommand>,
    },
}
struct CommandStreamUI {
    module_name_lookup: HashMap<*const u8, String>,
    selected_cmd_address: Option<*const kernel::ShaderCommand>,
}
impl CommandStreamUI {
    pub fn new() -> CommandStreamUI {
        let module_name_lookup: HashMap<_, _> = Module::get_all()
            .iter()
            .map(|m| {
                (
                    m.base as *const u8,
                    m.filename().unwrap_or_else(|| "unknown".to_string()),
                )
            })
            .collect();

        CommandStreamUI {
            module_name_lookup,
            selected_cmd_address: None,
        }
    }

    fn module_name_from_mba(&self, mba: *const u8) -> String {
        self.module_name_lookup
            .get(&mba)
            .cloned()
            .unwrap_or(format!("{:X?}", mba))
    }

    fn draw_cmd<PayloadType: Payload>(
        &mut self,
        index: usize,
        cmd: &Command<PayloadType>,
    ) -> anyhow::Result<()> {
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
                if let Some(address) = cmd.address {
                    ig::text("Pointer: ");
                    ig::same_line(None, Some(0.0));
                    if ig::small_button(&format!("{:X?}", address))? {
                        self.selected_cmd_address = Some(address);
                    }
                }
                ig::textf!("Timestamp: {:.3}ms", cmd.duration.as_secs_f64() * 1_000.0);
            }

            ig::separator();
            if ig::collapsing_header("Data", None, None)? {
                cmd.payload.draw()?;
            }

            #[allow(clippy::collapsible_if)]
            if ig::collapsing_header("Callstack", None, None)? {
                if ig::begin_table("xivr_debugger_callstack", 2, None, None, None)? {
                    ig::table_setup_column("Module", None, None, None)?;
                    ig::table_setup_column("Address", None, None, None)?;
                    ig::table_headers_row();

                    for frame in cmd.backtrace.frames().iter().skip(5) {
                        let mba = frame.module_base_address().unwrap_or(std::ptr::null_mut());
                        let address = unsafe { frame.ip().offset_from(mba) };

                        ig::table_next_row(None, None);
                        {
                            ig::table_next_column();
                            ig::text(&self.module_name_from_mba(mba as *const _));
                            ig::table_next_column();
                            let addr_str = format!("0x{:0width$X}", address, width = 6);
                            if ig::small_button(&addr_str)? {
                                ig::set_clipboard_text(&addr_str)?;
                            }
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

    pub fn draw_stream<PayloadType: Payload>(
        &mut self,
        title: &str,
        stream: &mut Stream<Command<PayloadType>>,
        select: bool,
    ) -> anyhow::Result<()> {
        if ig::begin_tab_item(
            title,
            None,
            if select {
                Some(ig::TabItemFlags::SetSelected)
            } else {
                None
            },
        )? {
            if ig::begin_child(
                &format!("Command Stream ({})", title),
                Some(ig::Vec2::new(300.0, 0.0)),
                Some(true),
                None,
            )? {
                for (i, cmd) in stream.stream.iter().enumerate() {
                    let is_selected = stream.selected_index == Some(i);
                    let name = format!("{}: {}", i, cmd.payload.title());

                    ig::push_style_color(ig::Col::Text, cmd.payload.colour());
                    if ig::selectable(&name, Some(is_selected), None, None)? {
                        stream.selected_index = Some(i);
                    }
                    ig::pop_style_color(1);

                    if is_selected {
                        ig::set_item_default_focus();
                    }
                }
                ig::end_child();
            }

            ig::same_line(None, None);
            if let Some(index) = stream.selected_index {
                let cmd = &stream.stream[index];
                self.draw_cmd(index, cmd)?;
            }
            ig::end_tab_item();
        }

        Ok(())
    }

    pub fn draw_captured(&mut self, state: &mut CommandStreamState) -> anyhow::Result<()> {
        if let CommandStreamState::Captured {
            shader_streams,
            processed_shader_stream,
            d3d_stream,
            message_stream,
        } = state
        {
            if ig::begin_tab_bar("xivr_debugger_command_stream_tabs", None)? {
                let mut selected_thread_id = None;
                for (thread_id, shader_stream) in shader_streams.iter_mut() {
                    if let Some(selected_cmd_address) = self.selected_cmd_address {
                        if let Some(index) = shader_stream
                            .stream
                            .iter()
                            .position(|cmd| cmd.address == Some(selected_cmd_address))
                        {
                            shader_stream.selected_index = Some(index);
                            selected_thread_id = Some(*thread_id);
                        }
                    }
                }

                if ig::begin_tab_item(
                    "Game",
                    None,
                    if selected_thread_id.is_some() {
                        Some(ig::TabItemFlags::SetSelected)
                    } else {
                        None
                    },
                )? {
                    if ig::begin_tab_bar("xivr_debugger_command_stream_tabs_game", None)? {
                        for (thread_id, shader_stream) in shader_streams {
                            let is_selected = selected_thread_id == Some(*thread_id);
                            if is_selected {
                                // Delay removal of the address until we're sure we've switched
                                self.selected_cmd_address = None;
                            }

                            self.draw_stream(
                                &format!("{}", thread_id),
                                shader_stream,
                                is_selected,
                            )?;
                        }
                        ig::end_tab_bar();
                    }
                    ig::end_tab_item();
                }
                self.draw_stream("Game (Processed)", processed_shader_stream, false)?;
                if ct_config::rendering::CAPTURE_D3D_COMMANDS {
                    self.draw_stream("D3D", d3d_stream, false)?;
                }
                self.draw_stream("Messages", message_stream, false)?;
                ig::end_tab_bar();
            }
        }

        Ok(())
    }

    fn draw(&mut self, state: &mut CommandStreamState) -> anyhow::Result<()> {
        {
            if ig::button("Capture", None)? {
                *state = CommandStreamState::WantToCapture;
            }
        }

        if let CommandStreamState::Captured { .. } = state {
            self.draw_captured(state)?;
        } else {
            ig::separator();
            ig::text("Capture a frame to proceed.");
        }

        Ok(())
    }
}

pub struct CommandStream {
    state: CommandStreamState,
    ui: CommandStreamUI,
}
impl CommandStream {
    pub fn new() -> CommandStream {
        CommandStream {
            state: CommandStreamState::Uncaptured,
            ui: CommandStreamUI::new(),
        }
    }

    pub fn pre_update(&mut self) -> anyhow::Result<()> {
        match self.state {
            CommandStreamState::WantToCapture => self.start_capture()?,
            CommandStreamState::Capturing { ref mut frames, .. } => {
                *frames += 1;
                if *frames == FRAMES_TO_CAPTURE {
                    self.end_capture()?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn start_capture(&mut self) -> anyhow::Result<()> {
        self.state = CommandStreamState::Capturing {
            start_instant: Instant::now(),
            shader_stream: vec![],
            processed_shader_stream: vec![],
            message_stream: vec![],
            d3d_stream: vec![],
            frames: 0,
        };

        Ok(())
    }

    pub fn end_capture(&mut self) -> anyhow::Result<()> {
        if let CommandStreamState::Capturing {
            shader_stream,
            processed_shader_stream,
            d3d_stream,
            message_stream,
            ..
        } = &self.state
        {
            let mut shader_streams = HashMap::new();
            for cmd in shader_stream {
                shader_streams
                    .entry(cmd.thread_id)
                    .or_insert(Stream {
                        stream: vec![],
                        selected_index: None,
                    })
                    .stream
                    .push(cmd.clone());
            }

            // It would be nice to move the Vecs here, but I cbf figuring it out
            self.state = CommandStreamState::Captured {
                shader_streams,
                processed_shader_stream: processed_shader_stream.clone().into(),
                d3d_stream: d3d_stream.clone().into(),
                message_stream: message_stream.clone().into(),
            };
        }
        Ok(())
    }

    pub fn is_capturing(&self) -> bool {
        matches!(self.state, CommandStreamState::Capturing { .. })
    }

    fn push_back_command_to_stream<PayloadType>(
        stream: &mut Vec<Command<PayloadType>>,
        address: Option<*const kernel::ShaderCommand>,
        start_instant: &Instant,
        payload: PayloadType,
    ) -> anyhow::Result<()> {
        use windows::Win32::System::Threading::GetCurrentThreadId;
        let backtrace = backtrace::Backtrace::new_unresolved();

        stream.push(Command::<PayloadType> {
            payload,
            address,
            backtrace,
            thread_id: unsafe { GetCurrentThreadId() },
            duration: Instant::now() - *start_instant,
        });

        Ok(())
    }

    fn push_back_command(
        &mut self,
        address: Option<*const kernel::ShaderCommand>,
        payload: ShaderPayload,
    ) -> anyhow::Result<()> {
        match &mut self.state {
            CommandStreamState::Capturing {
                shader_stream,
                start_instant,
                ..
            } => Self::push_back_command_to_stream(shader_stream, address, start_instant, payload),
            _ => Ok(()),
        }
    }

    fn shader_command_to_payload(cmd: &'static kernel::ShaderCommand) -> ShaderPayload {
        let cmd_type = unsafe { cmd.cmd_type() };
        match cmd_type {
            ShaderCommandType::SetRenderTargets => unsafe {
                let rts = cmd.payload().set_render_targets.get_render_target_slice();
                ShaderPayload::SetRenderTargets(rts.iter().map(|x| Ptr(*x)).collect())
            },
            ShaderCommandType::SetViewports => ShaderPayload::SetViewports,
            ShaderCommandType::SetViewportsFancy => ShaderPayload::SetViewportsFancy,
            ShaderCommandType::SetScissorRect => ShaderPayload::SetScissorRect,
            ShaderCommandType::Clear => ShaderPayload::Clear,
            ShaderCommandType::Draw => ShaderPayload::Draw,
            ShaderCommandType::DrawIndexed => ShaderPayload::DrawIndexed,
            ShaderCommandType::DrawIndexedInstanced => ShaderPayload::DrawIndexedInstanced,
            ShaderCommandType::DispatchComputeShader => ShaderPayload::DispatchComputeShader,
            ShaderCommandType::XIVRHijack => ShaderPayload::XIVRHijack,
            ShaderCommandType::CopyTexture => unsafe {
                let p = &cmd.payload().copy_texture;
                ShaderPayload::CopyTexture {
                    dst: Ptr(*p.dst_resource_ptr()),
                    src: Ptr(*p.src_resource_ptr()),
                }
            },
            ShaderCommandType::UnknownDraw => unsafe {
                let p = &cmd.payload().unknown_draw;
                ShaderPayload::UnknownDraw {
                    render_target: Ptr(*p.render_target_ptr()),
                    sampled_texture: Ptr(*p.sampled_texture_ptr()),
                }
            },
            ShaderCommandType::CopyResource => ShaderPayload::CopyResource,
            ShaderCommandType::ResetRendererMaybe => ShaderPayload::ResetRendererMaybe,
            ShaderCommandType::Unknown1 => ShaderPayload::Unknown1,
            ShaderCommandType::CopySubresourceRegion => ShaderPayload::CopySubresourceRegion,
            ShaderCommandType::SomethingWithStrings => ShaderPayload::SomethingWithStrings,
        }
    }

    pub fn add_command(&mut self, cmd: &'static kernel::ShaderCommand) -> anyhow::Result<()> {
        self.push_back_command(
            Some(cmd as *const kernel::ShaderCommand),
            Self::shader_command_to_payload(cmd),
        )
    }

    pub fn add_marker(&mut self, msg: String) -> anyhow::Result<()> {
        self.push_back_command(None, ShaderPayload::XIVRMarker(msg))
    }

    pub fn add_processed_command(
        &mut self,
        cmd: &'static kernel::ShaderCommand,
    ) -> anyhow::Result<()> {
        match &mut self.state {
            CommandStreamState::Capturing {
                processed_shader_stream,
                start_instant,
                ..
            } => Self::push_back_command_to_stream(
                processed_shader_stream,
                Some(cmd as *const kernel::ShaderCommand),
                start_instant,
                Self::shader_command_to_payload(cmd),
            ),
            _ => Ok(()),
        }
    }

    pub fn add_d3d_command(&mut self, payload: D3DPayload) -> anyhow::Result<()> {
        match &mut self.state {
            CommandStreamState::Capturing {
                d3d_stream,
                start_instant,
                ..
            } => Self::push_back_command_to_stream(d3d_stream, None, start_instant, payload),
            _ => Ok(()),
        }
    }

    pub fn add_message(&mut self, msg: String, submsgs: Vec<String>) -> anyhow::Result<()> {
        match &mut self.state {
            CommandStreamState::Capturing {
                message_stream,
                start_instant,
                ..
            } => Self::push_back_command_to_stream(message_stream, None, start_instant, (msg, submsgs)),
            _ => Ok(()),
        }
    }

    pub fn draw_ui(&mut self) -> anyhow::Result<()> {
        self.ui.draw(&mut self.state)
    }
}
