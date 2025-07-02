use std::sync::Arc;

use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::prelude::Backend;
use ratatui::Terminal;
use tui_commander::ui::Ui as CommanderUi;
use tui_commander::Commander;

use super::bindings::binder::Binder;
use super::context::TuiContext;
use super::error::AppError;
use super::jobserver::JobServer;
use super::widgets::boxes::Boxes;
use super::widgets::boxes::BoxesState;
use crate::tui::commands::TuiCommandContext;

#[allow(unused)]
pub struct App {
    tui_context: TuiContext,
    keybindings: Binder<AppState, AppError>,
    state: AppState,
    jobserver: JobServer,
}

pub(crate) struct AppState {
    commander: Commander<TuiCommandContext>,
    commander_ui: CommanderUi<TuiCommandContext>,
    do_exit: bool,
    boxes: Boxes,
    pub(crate) current_focus: FocusState,
    pub(crate) boxes_state: BoxesState,
    pub jobs_progress: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum FocusState {
    None,
    Commander,
    CommandMode,
}

impl App {
    pub fn new(tui_context: TuiContext) -> Self {
        Self {
            tui_context,

            keybindings: Binder::builder()
                .with_binding::<crate::tui::bindings::mappings::MoveLeft>()
                .with_binding::<crate::tui::bindings::mappings::MoveUp>()
                .with_binding::<crate::tui::bindings::mappings::MoveDown>()
                .with_binding::<crate::tui::bindings::mappings::MoveRight>()
                .with_binding::<crate::tui::bindings::mappings::ActivateCommander>()
                .with_binding::<crate::tui::bindings::mappings::Abort>()
                .build(),

            state: AppState {
                commander: Commander::builder()
                    .with_case_sensitive(false)
                    .with_command::<crate::tui::commands::quit::QuitCommand>()
                    .with_command::<crate::tui::commands::prev_message::PrevMessageCommand>()
                    .with_command::<crate::tui::commands::next_message::NextMessageCommand>()
                    .with_command::<crate::tui::commands::query::QueryCommand>()
                    .with_command::<crate::tui::commands::close::CloseCommand>()
                    .build(),
                commander_ui: CommanderUi::default(),
                current_focus: FocusState::None,
                do_exit: false,
                boxes: Boxes::empty(),
                boxes_state: BoxesState::default(),
                jobs_progress: Vec::new(),
            },
            jobserver: JobServer::default(),
        }
    }

    #[inline]
    pub fn add_box(&mut self, bx: Arc<crate::tui::model::MBox>) {
        self.state.boxes.add_box(bx);
        self.state.boxes_state.increase_boxes_count();
        self.state.boxes_state.focus_last();
    }

    #[inline]
    pub fn remove_currently_focused_box(&mut self) {
        self.state
            .boxes
            .remove_index(self.state.boxes_state.current_index());
        self.state.boxes_state.decrease_boxes_count();
    }

    pub fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<(), AppError> {
        loop {
            if self.state.do_exit {
                tracing::info!("Shutting down TUI");
                break Ok(());
            }

            if let Some(mut ready_job) = self.jobserver.get_next_ready_job() {
                ready_job.finalize(&mut self);
            }

            self.state.jobs_progress = self.jobserver.progress_states();
            terminal.draw(|frame| self.draw(frame))?;

            match crossterm::event::poll(std::time::Duration::from_millis(50)) {
                Ok(true) => {
                    tracing::debug!("Polling resulted in found event");
                    let event = crossterm::event::read()?;
                    tracing::debug!(?event, "Found event");
                    if let Some(command) = self.handle_tui_event(event) {
                        self.handle_app_message(command)?;
                    }
                    tracing::debug!("Event handled");
                }
                Ok(false) => {
                    tracing::trace!("Polling resulted in no event");
                }
                Err(error) => {
                    tracing::warn!(?error, "Polling failed");
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        if self.state.jobs_progress.is_empty() {
            frame.render_stateful_widget(
                &mut self.state.boxes,
                frame.area(),
                &mut self.state.boxes_state,
            );
        } else {
            let [boxes_area, progress_area] =
                Layout::vertical([Constraint::Fill(1), Constraint::Length(1)])
                    .flex(ratatui::layout::Flex::Start)
                    .areas(frame.area());

            frame.render_stateful_widget(
                &mut self.state.boxes,
                boxes_area,
                &mut self.state.boxes_state,
            );

            let constraint = Constraint::from_lengths(self.state.jobs_progress.iter().map(|_| 1));
            let _layout = Layout::vertical(constraint).split(progress_area);
            // TODO: Fill 'layout' with progress bars, one for each self.state.jobs_progress
        }
        if self.state.current_focus == FocusState::Commander {
            frame.render_stateful_widget(
                &mut self.state.commander_ui,
                frame.area(),
                &mut self.state.commander,
            );
        }
    }

    fn handle_tui_event(&mut self, event: crossterm::event::Event) -> Option<AppMessage> {
        match event {
            ratatui::crossterm::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    tracing::trace!(?key, "Event is keypress");

                    match self.state.current_focus {
                        FocusState::None | FocusState::CommandMode => {
                            match self.keybindings.run_binding_for_keycode(
                                key.code,
                                key.modifiers,
                                &mut self.state,
                            ) {
                                Some(Ok(())) => return Some(AppMessage::KeyBindingSuccessfull),
                                Some(Err(error)) => {
                                    return Some(AppMessage::KeyBindingErrored(error))
                                }
                                None => {
                                    // No keybinding found
                                    return Some(AppMessage::UnboundKey(key));
                                }
                            }
                        }

                        FocusState::Commander => match key.code {
                            KeyCode::Esc => {
                                tracing::debug!("Deactivating EX");
                                self.state.current_focus = FocusState::CommandMode;
                            }
                            KeyCode::Enter => {
                                let mut tui_commander_context =
                                    crate::tui::commands::TuiCommandContext {
                                        command_to_execute: None,
                                    };

                                match self.state.commander.execute(&mut tui_commander_context) {
                                    Ok(()) => {
                                        tracing::debug!("Commander context executed successfully");
                                    }
                                    Err(error) => {
                                        tracing::error!(
                                            ?error,
                                            "Commander context execution failed"
                                        );
                                    }
                                }

                                self.state.current_focus = FocusState::CommandMode;
                                self.state.commander_ui.reset();
                                return tui_commander_context.command_to_execute;
                            }
                            _ => {
                                tracing::debug!(?key, "Forwarding keypress to commander UI");
                                self.state.commander_ui.handle_key_press(key);

                                tracing::debug!(?key, "Setting commander input");
                                self.state
                                    .commander
                                    .set_input(self.state.commander_ui.value().to_string());
                            }
                        },
                    }
                }
            }
            _ => {
                tracing::trace!(?event, "Unhandled TUI event");
            }
        }

        None
    }

    fn handle_app_message(&mut self, message: AppMessage) -> Result<(), AppError> {
        match message {
            AppMessage::Quit => {
                self.state.do_exit = true;
            }

            AppMessage::NextMessage => {
                tracing::info!("Next Sidebar Entry command received");
                self.state.boxes_state.focus_next();
            }

            AppMessage::PrevMessage => {
                tracing::info!("Prev Sidebar Entry command received");
                self.state.boxes_state.focus_prev();
            }

            AppMessage::Query(args) => {
                let query = args.join(" ");
                tracing::info!(?query, "Query received");

                self.jobserver
                    .add_job(crate::tui::jobserver::query::QueryJob::new(
                        query,
                        self.tui_context.notmuch.clone(),
                    ));
            }

            AppMessage::Close => {
                tracing::debug!("Closing current tab");
                self.remove_currently_focused_box();
            }
            AppMessage::KeyBindingSuccessfull => {
                tracing::debug!("Keybinding executed successfully");
            }
            AppMessage::KeyBindingErrored(error) => {
                tracing::error!(?error, "Keybinding failed");
            }
            AppMessage::UnboundKey(key) => {
                tracing::error!(?key, "Unbound key");
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum AppMessage {
    Quit,
    PrevMessage,
    NextMessage,
    Query(Vec<String>),
    Close,
    KeyBindingSuccessfull,
    KeyBindingErrored(AppError),
    UnboundKey(crossterm::event::KeyEvent),
}
