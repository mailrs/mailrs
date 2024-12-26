use nucleo_matcher::Matcher;

mod test;

#[derive(Default)]
pub struct Commander {
    commands: Vec<Box<dyn DynamicCommand>>,
    search_engine: Matcher,
    current_input: String,
}

impl Commander {
    pub fn with_command(mut self, command: Box<dyn DynamicCommand>) -> Self {
        self.commands.push(command);
        self
    }

    pub fn set_input(&mut self, input: String) {
        self.current_input = input;
    }

    pub fn suggestions(&mut self) -> Vec<&str> {
        let suggestions = nucleo_matcher::pattern::Pattern::new(
            &self.current_input,
            nucleo_matcher::pattern::CaseMatching::Ignore,
            nucleo_matcher::pattern::Normalization::Never,
            nucleo_matcher::pattern::AtomKind::Fuzzy,
        )
        .match_list(
            self.commands.iter().map(|c| c.name()),
            &mut self.search_engine,
        );

        tracing::debug!(n = %suggestions.len(), command = ?self.current_input, "Searched command");

        suggestions.into_iter().map(|tpl| tpl.0).collect()
    }

    pub fn execute_current_suggestion(&mut self) -> Result<(), CommandError> {
        // let Some(current_suggestions) = self.suggestions().first() else {
        //     return Err(CommandError::NoSuggestion);
        // };
        todo!()
    }

    pub fn clear(&mut self) {
        todo!()
    }
}

pub trait Command: Send + 'static {
    type Error: std::error::Error;
    type Arg: Argument<Error = Self::Error>;

    const NAME: &'static str;

    fn name(&self) -> &'static str {
        Self::NAME
    }

    fn execute(&self, args: Vec<Self::Arg>) -> Result<(), Self::Error>;
}

trait EraseCommand {
    fn erase(self) -> ErasedCommand;
}

impl<C: Command> EraseCommand for C {
    fn erase(self) -> ErasedCommand {
        fn command_handler<C>(
            command: &dyn DynamicCommand,
            args: Vec<ErasedArgument>,
        ) -> Result<(), CommandError>
        where
            C: Command,
        {
            let command = match command.downcast_ref::<C>() {
                Some(command) => command,
                None => panic!("Bug"),
            };

            let args = args
                .into_iter()
                .map(|arg| match arg.argument.downcast::<C::Arg>() {
                    Ok(a) => a,
                    Err(_) => panic!("Bug"),
                })
                .map(|a| *a)
                .collect::<Vec<C::Arg>>();
            command
                .execute(args)
                .map_err(Box::new)
                .map_err(|e| CommandError(e))
        }

        ErasedCommand {
            command: Box::new(self),
            command_handler: command_handler::<C>,
        }
    }
}

trait DynamicCommand: Send + downcast_rs::Downcast + 'static {
    fn name(&self) -> &'static str;
}

downcast_rs::impl_downcast!(DynamicCommand);

impl<C: Command> DynamicCommand for C {
    fn name(&self) -> &'static str {
        C::NAME
    }
}

type BoxedCommand = Box<dyn DynamicCommand>;

type CommandHandlerFn =
    for<'r> fn(&'r dyn DynamicCommand, Vec<ErasedArgument>) -> Result<(), CommandError>;

struct ErasedCommand {
    command: BoxedCommand,
    command_handler: CommandHandlerFn,
}

impl ErasedCommand {
    pub fn execute(&self, args: Vec<ErasedArgument>) -> Result<(), CommandError> {
        (self.command_handler)(&*self.command, args)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Command error")]
pub struct CommandError(#[from] Box<dyn std::error::Error>);

pub trait Argument
where
    Self: Sized + Send + 'static,
{
    type Error: std::error::Error;

    fn build_from_str(s: &str) -> Result<Option<Self>, Self::Error>;
}

pub trait DynamicArgument: Send + downcast_rs::Downcast + 'static {}

downcast_rs::impl_downcast!(DynamicArgument);

impl<A: Argument> DynamicArgument for A {}

type BoxedArgument = Box<dyn DynamicArgument>;

struct ErasedArgument {
    argument: BoxedArgument,
}
