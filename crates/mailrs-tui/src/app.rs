use std::sync::Arc;

use futures::FutureExt;
use futures::StreamExt;
use ratatui::crossterm::event::EventStream;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::layout::Constraint;
use ratatui::layout::Layout;
use ratatui::prelude::Backend;
use ratatui::Terminal;
use tui_commander::ui::Ui as CommanderUi;
use tui_commander::Commander;

use super::bindings::binder::Binder;
use super::context::TuiContext;
use super::widgets::boxes::BoxesState;
use crate::commands::TuiCommandContext;
use crate::focus::Focus;
use crate::widgets::logger::LoggerState;

#[allow(unused)]
pub struct App {
    tui_context: TuiContext,
    keybindings: Binder<AppState, crate::error::Error>,
    state: AppState,
}

pub(crate) struct AppState {
    pub(crate) commander: Commander<TuiCommandContext>,
    pub(crate) commander_ui: CommanderUi<TuiCommandContext>,
    do_exit: bool,
    pub(crate) show_logger: bool,
    pub(crate) current_focus: Focus,
    pub(crate) boxes_state: BoxesState,
    pub(crate) logger_state: LoggerState,
}

impl App {
    pub fn new(initial_box: Arc<crate::model::MBox>, tui_context: TuiContext) -> Self {
        Self {
            tui_context,

            keybindings: {
                use crate::bindings::mappings::commander;
                use crate::bindings::mappings::logger;
                use crate::bindings::mappings::mbox;
                use crate::bindings::mappings::movement;

                Binder::new()
                    .with_binding::<commander::ActivateCommander>()
                    .with_binding::<commander::DeactivateCommander>()
                    .with_binding::<commander::RunCommander>()
                    .with_binding::<logger::ShowLogger>()
                    .with_binding::<movement::MoveDown>()
                    .with_binding::<movement::MoveLeft>()
                    .with_binding::<movement::MoveRight>()
                    .with_binding::<movement::MoveUp>()
                    .with_binding::<mbox::NextMail>()
                    .with_binding::<mbox::PrevMail>()
                    .with_binding::<mbox::NextBox>()
                    .with_binding::<mbox::PrevBox>()
                    .with_binding::<mbox::OpenMessage>()
                    .with_binding::<mbox::CloseMessage>()
            },

            state: AppState {
                commander: Commander::builder()
                    .with_case_sensitive(false)
                    .with_command::<crate::commands::quit::QuitCommand>()
                    .with_command::<crate::commands::prev_message::PrevMessageCommand>()
                    .with_command::<crate::commands::next_message::NextMessageCommand>()
                    .with_command::<crate::commands::query::QueryCommand>()
                    .with_command::<crate::commands::close::CloseCommand>()
                    .build(),
                commander_ui: CommanderUi::default(),
                show_logger: false,
                current_focus: Focus::Box,
                do_exit: false,
                boxes_state: BoxesState::new(initial_box),
                logger_state: LoggerState::new(tracing::level_filters::LevelFilter::current()),
            },
        }
    }

    #[inline]
    pub fn add_box(&mut self, bx: Arc<crate::model::MBox>) {
        self.state.boxes_state.add_box(bx);
        self.state.boxes_state.focus_last();
    }

    #[inline]
    pub fn remove_currently_focused_box(&mut self) {
        self.state.boxes_state.remove_current_box();
    }

    pub async fn run(
        mut self,
        mut terminal: Terminal<impl Backend>,
    ) -> Result<(), crate::error::Error> {
        let mut events = EventStream::new();
        loop {
            if self.state.do_exit {
                tracing::info!("Shutting down TUI");
                break Ok(());
            }

            terminal.draw(|frame| self.draw(frame))?;

            let command_to_execute = tokio::select! {
                input_event = events.next().fuse() => {
                    let input = input_event.unwrap().unwrap();
                    self.handle_tui_event(input).await
                }
            };

            if let Some(command) = command_to_execute {
                self.handle_app_message(command).await?;
            }
        }
    }

    fn draw(&mut self, frame: &mut ratatui::Frame<'_>) {
        let main_area = if self.state.show_logger {
            let [main_area, logger_area] =
                Layout::vertical([Constraint::Percentage(70), Constraint::Percentage(30)])
                    .areas(frame.area());

            frame.render_stateful_widget(
                super::widgets::logger::Logger,
                logger_area,
                &mut self.state.logger_state,
            );
            main_area
        } else {
            frame.area()
        };

        frame.render_stateful_widget(
            &mut super::widgets::boxes::Boxes,
            main_area,
            &mut self.state.boxes_state,
        );
        if self.state.current_focus == Focus::Commander {
            frame.render_stateful_widget(
                &mut self.state.commander_ui,
                main_area,
                &mut self.state.commander,
            );
        }
    }

    async fn handle_tui_event(
        &mut self,
        event: ratatui::crossterm::event::Event,
    ) -> Option<AppMessage> {
        match event {
            ratatui::crossterm::event::Event::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    tracing::trace!(
                        focus = ?self.state.current_focus,
                        ?key,
                        "Processing keybinding"
                    );

                    match self.keybindings.run_binding_for_keycode(
                        self.state.current_focus,
                        key.code,
                        key.modifiers,
                        &mut self.state,
                    ) {
                        Some(Ok(opt_app_message)) => return opt_app_message,
                        Some(Err(error)) => return Some(AppMessage::KeyBindingErrored(error)),
                        None => {
                            if self.state.current_focus == Focus::Commander {
                                tracing::debug!(?key, "Forwarding keypress to commander UI");
                                self.state.commander_ui.handle_key_press(key);

                                tracing::debug!(?key, "Setting commander input");
                                self.state
                                    .commander
                                    .set_input(self.state.commander_ui.value().to_string());
                            } else {
                                // No keybinding found
                                return Some(AppMessage::UnboundKey(key));
                            }
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

    async fn handle_app_message(&mut self, message: AppMessage) -> Result<(), crate::error::Error> {
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
                use crate::model::MBox;
                use crate::model::Message;
                use crate::model::Tag;

                let query = args.join(" ");
                tracing::info!(?query, "Query received");

                let messages = self
                    .tui_context
                    .notmuch
                    .create_query(&query)
                    .search_messages()
                    .await
                    .map_err(crate::error::Error::from)?
                    .into_iter()
                    .map(|message| {
                        let notmuch = self.tui_context.notmuch.clone();

                        async move {
                            let tags = notmuch
                                .clone()
                                .tags_for_message(&message)
                                .await?
                                .unwrap_or_default();

                            let from = match message.header("From").await {
                                Ok(someornone) => someornone,
                                Err(error) => {
                                    tracing::error!(
                                        ?error,
                                        id = message.id(),
                                        "Failed to fetch 'From' header for message"
                                    );
                                    None
                                }
                            };

                            let subject = match message.header("Subject").await {
                                Ok(someornone) => someornone,
                                Err(error) => {
                                    tracing::error!(
                                        ?error,
                                        id = message.id(),
                                        "Failed to fetch 'Subject' header for message"
                                    );
                                    None
                                }
                            };

                            tracing::info!(id = ?message.id(), ?tags, "Found message");

                            Ok(Message {
                                id: message.id().to_string(),
                                from,
                                subject,
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
                    .collect::<Vec<Result<Message, crate::error::Error>>>()
                    .await
                    .into_iter()
                    .collect::<Result<Vec<_>, _>>()?;

                let mbox = MBox::new(query, messages);
                self.add_box(Arc::new(mbox));
            }

            AppMessage::Close => {
                tracing::debug!("Closing current tab");
                self.remove_currently_focused_box();
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
    KeyBindingErrored(crate::error::Error),
    UnboundKey(ratatui::crossterm::event::KeyEvent),
}
