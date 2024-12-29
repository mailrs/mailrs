use super::TuiCommandContext;
use crate::tui::app::AppMessage;

#[derive(Debug)]
pub struct PrevMessageCommand;

impl std::fmt::Display for PrevMessageCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "prev")
    }
}

impl tui_commander::Command<TuiCommandContext> for PrevMessageCommand {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "prev"
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
        if !arguments.is_empty() {
            return Err(format!(
                "The 'prev' command does not support arguments: '{}'",
                arguments.join(" ")
            )
            .into());
        }
        context.command_to_execute = Some(AppMessage::PrevMessage);
        Ok(())
    }
}
