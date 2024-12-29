use super::TuiCommandContext;
use crate::tui::app::AppMessage;

#[derive(Debug)]
pub struct QueryCommand;

impl std::fmt::Display for QueryCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "query")
    }
}

impl tui_commander::Command<TuiCommandContext> for QueryCommand {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "query"
    }

    fn build_from_command_name_str(
        _input: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn args_are_valid(_args: &[&str]) -> bool
    where
        Self: Sized,
    {
        false // no arguments supported
    }

    fn execute(
        &self,
        arguments: Vec<String>,
        context: &mut TuiCommandContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        // TODO: Parse whether arguments are correct notmuch query?
        context.command_to_execute = Some(AppMessage::Query(arguments));
        Ok(())
    }
}
