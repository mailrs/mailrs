#![allow(dead_code)]

use nucleo_matcher::Matcher;

mod test;

#[derive(Default)]
pub struct Commander {
    commands: Vec<ErasedCommand>,
    search_engine: Matcher,
    current_input: Vec<String>,

    do_suggest_on_empty_input: bool,
}

impl Commander {
    pub fn with_command<C>(mut self, command: C) -> Self
    where
        C: Command,
    {
        self.commands.push(command.erase());
        self
    }

    pub fn set_input(&mut self, input: Vec<String>) {
        self.current_input = input;
    }

    pub fn suggestions(&mut self) -> Vec<String> {
        match self.current_input.len() {
            0 => {
                if self.do_suggest_on_empty_input {
                    self.commands.iter().map(|c| c.name().to_string()).collect()
                } else {
                    Vec::new()
                }
            }

            1 => {
                // suggest a command
                let suggestions = nucleo_matcher::pattern::Pattern::new(
                    self.current_input.first().unwrap(),
                    nucleo_matcher::pattern::CaseMatching::Ignore,
                    nucleo_matcher::pattern::Normalization::Never,
                    nucleo_matcher::pattern::AtomKind::Fuzzy,
                )
                .match_list(
                    self.commands.iter().map(|c| c.name()),
                    &mut self.search_engine,
                );

                tracing::debug!(n = %suggestions.len(), command = ?self.current_input, "Searched command");

                suggestions
                    .into_iter()
                    .map(|tpl| tpl.0.to_string())
                    .collect()
            }

            _ => {
                // suggest an argument for the current command

                // safe because of above check:
                let current_input_command = self.current_input.first().unwrap();
                let Some(current_command) = self
                    .commands
                    .iter()
                    .find(|c| c.name() == current_input_command)
                else {
                    // No current command found, suggest nothing
                    return Vec::new();
                };

                current_command.arg_suggestions()
            }
        }
    }

    pub fn execute_current_input(&mut self) -> Result<(), CommandError> {
        let mut it = self.current_input.iter();

        let Some(command) = it.next() else {
            return Err(CommandError::NoInput);
        };

        let Some(command) = self.commands.iter().find(|c| c.name() == command) else {
            return Err(CommandError::UnknownCommand);
        };

        let args = it.cloned().collect();

        command.execute(args)
    }

    pub fn clear(&mut self) {
        todo!()
    }
}

pub trait Command: Send + 'static {
    type Error: std::error::Error;
    const NAME: &'static str;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn arg_suggestions() -> Vec<String>;
    fn execute(&self, args: Vec<String>) -> Result<(), Self::Error>;
}

trait EraseCommand {
    fn erase(self) -> ErasedCommand;
}

impl<C: Command> EraseCommand for C {
    fn erase(self) -> ErasedCommand {
        fn command_handler<C>(
            command: &dyn DynamicCommand,
            args: Vec<String>,
        ) -> Result<(), CommandError>
        where
            C: Command,
        {
            let command = match command.downcast_ref::<C>() {
                Some(command) => command,
                None => panic!("Bug"),
            };

            command
                .execute(args)
                .map_err(Box::new)
                .map_err(|e| CommandError::CommandImpl(e))
        }

        ErasedCommand {
            command: Box::new(self),
            command_handler: command_handler::<C>,
        }
    }
}

pub trait DynamicCommand: Send + downcast_rs::Downcast + 'static {
    fn name(&self) -> &'static str;
    fn arg_suggestions(&self) -> Vec<String>;
}

downcast_rs::impl_downcast!(DynamicCommand);

impl<C: Command> DynamicCommand for C {
    fn name(&self) -> &'static str {
        C::NAME
    }

    fn arg_suggestions(&self) -> Vec<String> {
        C::arg_suggestions()
    }
}

type BoxedCommand = Box<dyn DynamicCommand>;

type CommandHandlerFn =
    for<'r> fn(&'r dyn DynamicCommand, Vec<String>) -> Result<(), CommandError>;

struct ErasedCommand {
    command: BoxedCommand,
    command_handler: CommandHandlerFn,
}

impl ErasedCommand {
    pub fn execute(&self, args: Vec<String>) -> Result<(), CommandError> {
        (self.command_handler)(&*self.command, args)
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.command.name()
    }

    #[inline]
    pub fn arg_suggestions(&self) -> Vec<String> {
        self.command.arg_suggestions()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CommandError {
    #[error("Command error")]
    CommandImpl(#[from] Box<dyn std::error::Error>),

    #[error("Unknown command")]
    UnknownCommand,

    #[error("No command input")]
    NoInput,
}
