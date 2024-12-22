use std::str::FromStr;

use crossterm::event::KeyCode;
use crossterm::event::KeyEvent;
use nucleo_matcher::Matcher;

use super::app::AppMessage;
use super::widgets::commander::CommanderUi;

pub struct Commander {
    ui: CommanderUi,
    sender: tokio::sync::mpsc::Sender<AppMessage>,

    search_engine: Matcher,
    commands: Vec<String>,
}

impl Commander {
    pub fn new(command_sender: tokio::sync::mpsc::Sender<AppMessage>) -> Self {
        let search_engine = Matcher::new(nucleo_matcher::Config::DEFAULT);

        let commands = enum_iterator::all::<Command>()
            .map(|ex| ex.to_string())
            .collect();

        Self {
            commands,
            sender: command_sender,
            ui: CommanderUi::default(),
            search_engine,
        }
    }

    pub fn ui(&self) -> &CommanderUi {
        &self.ui
    }

    pub fn activate(&mut self) {
        self.ui.activate();
    }

    pub fn is_activated(&self) -> bool {
        self.ui.is_activated()
    }

    pub fn clear(&mut self) {
        self.ui.clear();
    }

    pub async fn handle_key_press(&mut self, event: KeyEvent) {
        match event.code {
            KeyCode::Esc => {
                self.ui.clear();
            }

            KeyCode::Enter => {
                tracing::trace!(command = ?self.ui.value(), "Processing command");

                let command_str = {
                    let mut val = self.ui.value().split(' ');
                    let Some(cmd) = val.next() else {
                        self.ui.clear();
                        return;
                    };
                    cmd
                };

                let found = nucleo_matcher::pattern::Pattern::new(
                    command_str,
                    nucleo_matcher::pattern::CaseMatching::Ignore,
                    nucleo_matcher::pattern::Normalization::Never,
                    nucleo_matcher::pattern::AtomKind::Fuzzy,
                )
                .match_list(&self.commands, &mut self.search_engine);

                tracing::debug!(n = %found.len(), ?command_str, "Searched command");

                if found.is_empty() {
                    tracing::debug!(command = ?self.ui.value(), "Command is unknown");
                    self.ui.unknown_command = true;
                    return;
                }

                let first = found.first().cloned();
                self.ui.suggestions = found.into_iter().map(|(s, _)| s.to_string()).collect();

                if let Some(command) = first {
                    let Ok(command) = Command::from_str(command.0) else {
                        panic!("Bug")
                    };

                    tracing::debug!(?command, "Sending command to app");
                    let _ = self.sender.send(AppMessage::Command(command)).await;
                    self.ui.clear()
                }
            }

            _ => {
                self.ui.handle_key_press(event);
                let Some(command_str) = self.ui.value().split(' ').next() else {
                    return;
                };

                let found = nucleo_matcher::pattern::Pattern::new(
                    command_str,
                    nucleo_matcher::pattern::CaseMatching::Ignore,
                    nucleo_matcher::pattern::Normalization::Never,
                    nucleo_matcher::pattern::AtomKind::Fuzzy,
                )
                .match_list(&self.commands, &mut self.search_engine);
                tracing::debug!(n = %found.len(), ?command_str, "Searched command");
                self.ui.suggestions = found.into_iter().map(|(s, _)| s.to_string()).collect();
            }
        }
    }
}

#[derive(
    Debug,
    Copy,
    Clone,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    Hash,
    derive_more::Display,
    enum_iterator::Sequence,
)]
pub enum Command {
    #[display("quit")]
    Quit,
    #[display("next")]
    NextMessage,
    #[display("previous")]
    PrevMessage,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        match s {
            "quit" => Ok(Quit),
            "next" => Ok(NextMessage),
            "previous" => Ok(PrevMessage),

            _other => Err(()),
        }
    }
}
