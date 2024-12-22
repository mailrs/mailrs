use crossterm::event::EventStream;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use futures::FutureExt;
use futures::StreamExt;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::prelude::Backend;
use ratatui::Terminal;

use super::commander::Commander;
use super::context::TuiContext;
use super::error::AppError;
use super::model::MBox;
use super::widgets::boxes::Boxes;
use super::widgets::boxes::BoxesState;

pub struct App {
    commander: Commander,
    boxes: Boxes,
    command_receiver: tokio::sync::mpsc::Receiver<AppMessage>,
    do_exit: bool,
    current_focus: FocusState,
    boxes_state: BoxesState,
    tui_context: TuiContext,
}

#[derive(Debug, PartialEq, Eq)]
enum FocusState {
    None,
    Commander,
    CommandMode,
}

impl App {
    pub fn new(tui_context: TuiContext, initial_box: MBox) -> Self {
        let (command_sender, command_receiver) = tokio::sync::mpsc::channel(1);

        Self {
            commander: Commander::new(command_sender),
            command_receiver,
            do_exit: false,
            current_focus: FocusState::None,
            boxes: Boxes::new(initial_box),
            boxes_state: BoxesState::default(),
            tui_context,
        }
    }

    pub async fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<(), AppError> {
        let mut events = EventStream::new();
        loop {
            if self.do_exit {
                tracing::info!("Shutting down TUI");
                break Ok(());
            }

            terminal.draw(|frame| self.draw(frame))?;

            tokio::select! {
                input_event = events.next().fuse() => {
                    let input = input_event.unwrap().unwrap();
                    tracing::trace!("Event available = {:?}", input);
                    self.handle_tui_event(input).await;
                }
            }

            let _ = self
                .command_receiver
                .try_recv()
                .map(Some)
                .or_else(|err| match err {
                    tokio::sync::mpsc::error::TryRecvError::Empty => Ok(None),
                    tokio::sync::mpsc::error::TryRecvError::Disconnected => {
                        Err(AppError::InternalChannelClosed)
                    }
                })?
                .map(|m| {
                    tracing::debug!("Received command, handling");
                    self.handle_app_message(m)
                })
                .transpose()?;
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        let [main_area, commander_area] =
            Layout::vertical([Constraint::Percentage(100), Constraint::Min(6)]).areas(frame.area());

        frame.render_stateful_widget(&self.boxes, main_area, &mut self.boxes_state);
        frame.render_widget(self.commander.ui(), commander_area);
    }

    async fn handle_tui_event(&mut self, event: crossterm::event::Event) {
        match event {
            ratatui::crossterm::event::Event::Key(key) => {
                tracing::trace!(?key, "Event is keypress");
                if self.current_focus == FocusState::Commander {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                        tracing::debug!("Deactivating EX");
                        self.current_focus = FocusState::CommandMode;
                        self.commander.clear();
                    } else {
                        tracing::debug!(?key, "Sending key to EX");
                        self.commander.handle_key_press(key).await;
                        if !self.commander.is_activated() {
                            self.current_focus = FocusState::CommandMode;
                        }
                    }
                } else if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(':') => {
                            tracing::debug!("Activating EX");
                            self.current_focus = FocusState::Commander;
                            self.commander.activate();
                        }

                        KeyCode::Up => {}

                        KeyCode::Down => {}

                        _ => {
                            tracing::debug!(key = ?key.code, "Unhandled key press");
                        }
                    }
                }
            }
            _ => {
                tracing::trace!(?event, "Unhandled TUI event");
            }
        }
    }

    fn handle_app_message(&mut self, message: AppMessage) -> Result<(), AppError> {
        match message {
            AppMessage::Shutdown(_message) => {
                self.do_exit = true;
            }
            AppMessage::Command(crate::tui::commander::Command::Quit) => {
                tracing::info!("Quit Command received");
                self.do_exit = true;
            }

            AppMessage::Command(crate::tui::commander::Command::NextMessage) => {
                tracing::info!("Next Sidebar Entry command received");
            }

            AppMessage::Command(crate::tui::commander::Command::PrevMessage) => {
                tracing::info!("Prev Sidebar Entry command received");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum AppMessage {
    #[allow(dead_code)]
    Shutdown(String),
    Command(crate::tui::commander::Command),
}
