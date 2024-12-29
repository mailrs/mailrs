use std::sync::Arc;

use crossterm::event::EventStream;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use futures::FutureExt;
use futures::StreamExt;
use ratatui::prelude::Backend;
use ratatui::Terminal;
use tui_commander::ui::Ui as CommanderUi;
use tui_commander::Commander;

use super::context::TuiContext;
use super::error::AppError;
use super::widgets::boxes::Boxes;
use super::widgets::boxes::BoxesState;
use crate::tui::commands::TuiCommandContext;

#[allow(unused)]
pub struct App {
    commander: Commander<TuiCommandContext>,
    commander_ui: CommanderUi<TuiCommandContext>,
    do_exit: bool,
    boxes: Boxes,
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
    pub fn new(tui_context: TuiContext) -> Self {
        Self {
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
            tui_context,
        }
    }

    #[inline]
    pub fn add_box(&mut self, bx: Arc<crate::tui::model::MBox>) {
        self.boxes.add_box(bx);
        self.boxes_state.increase_boxes_count()
    }

    #[inline]
    pub fn remove_currently_focused_box(&mut self) {
        self.boxes.remove_index(self.boxes_state.current_index());
        self.boxes_state.decrease_boxes_count();
    }

    pub async fn run(mut self, mut terminal: Terminal<impl Backend>) -> Result<(), AppError> {
        let mut events = EventStream::new();
        loop {
            if self.do_exit {
                tracing::info!("Shutting down TUI");
                break Ok(());
            }

            terminal.draw(|frame| self.draw(frame))?;

            let command_to_execute = tokio::select! {
                input_event = events.next().fuse() => {
                    let input = input_event.unwrap().unwrap();
                    tracing::trace!("Event available = {:?}", input);
                    self.handle_tui_event(input).await
                }
            };

            if let Some(command) = command_to_execute {
                self.handle_app_message(command).await?;
            }
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        frame.render_stateful_widget(&mut self.boxes, frame.area(), &mut self.boxes_state);
        if self.current_focus == FocusState::Commander {
            frame.render_stateful_widget(&mut self.commander_ui, frame.area(), &mut self.commander);
        }
    }

    async fn handle_tui_event(&mut self, event: crossterm::event::Event) -> Option<AppMessage> {
        match event {
            ratatui::crossterm::event::Event::Key(key) => {
                tracing::trace!(?key, "Event is keypress");
                if self.current_focus == FocusState::Commander {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Esc => {
                                tracing::debug!("Deactivating EX");
                                self.current_focus = FocusState::CommandMode;
                            }
                            KeyCode::Enter => {
                                let mut tui_commander_context =
                                    crate::tui::commands::TuiCommandContext {
                                        command_to_execute: None,
                                    };

                                match self.commander.execute(&mut tui_commander_context) {
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

                                self.current_focus = FocusState::CommandMode;
                                self.commander_ui.reset();
                                return tui_commander_context.command_to_execute;
                            }
                            _ => {
                                tracing::debug!(?key, "Forwarding keypress to commander UI");
                                self.commander_ui.handle_key_press(key);

                                tracing::debug!(?key, "Setting commander input");
                                self.commander
                                    .set_input(self.commander_ui.value().to_string());
                            }
                        }
                    }
                } else if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char(':') => {
                            tracing::debug!("Activating EX");
                            self.current_focus = FocusState::Commander;
                        }

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

        None
    }

    async fn handle_app_message(&mut self, message: AppMessage) -> Result<(), AppError> {
        match message {
            AppMessage::Quit => {
                self.do_exit = true;
            }

            AppMessage::NextMessage => {
                tracing::info!("Next Sidebar Entry command received");
                self.boxes_state.focus_next();
            }

            AppMessage::PrevMessage => {
                tracing::info!("Prev Sidebar Entry command received");
                self.boxes_state.focus_prev();
            }

            AppMessage::Query(args) => {
                use crate::tui::model::MBox;
                use crate::tui::model::Message;
                use crate::tui::model::Tag;

                let query = args.join(" ");
                tracing::info!(?query, "Query received");

                let messages = self
                    .tui_context
                    .notmuch
                    .create_query(&query)
                    .search_messages()
                    .await
                    .map_err(AppError::from)?
                    .into_iter()
                    .map(|message| {
                        let notmuch = self.tui_context.notmuch.clone();

                        async move {
                            let tags = notmuch
                                .clone()
                                .tags_for_message(&message)
                                .await?
                                .unwrap_or_default();

                            tracing::info!(id = ?message.id(), ?tags, "Found message");

                            Ok(Message {
                                id: message.id().to_string(),
                                tags: tags
                                    .into_iter()
                                    .map(|name| Tag {
                                        name: name.to_string(),
                                    })
                                    .collect::<Vec<Tag>>(),
                            })
                        }
                    })
                    .collect::<futures::stream::FuturesUnordered<_>>()
                    .collect::<Vec<Result<Message, crate::notmuch::WorkerError<_>>>>()
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(AppError::from)?;

                let mbox = MBox::new(query, messages);
                self.add_box(Arc::new(mbox));
            }

            AppMessage::Close => {
                tracing::debug!("Closing current tab");
                self.remove_currently_focused_box();
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
}
