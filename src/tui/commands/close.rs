use super::TuiCommandContext;
use crate::tui::app::AppMessage;

#[derive(Debug)]
pub struct CloseCommand;

impl std::fmt::Display for CloseCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "close")
    }
}

impl tui_commander::Command<TuiCommandContext> for CloseCommand {
    fn name() -> &'static str
    where
        Self: Sized,
    {
        "close"
    }

    fn build_from_command_name_str(
        _input: &str,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync + 'static>>
    where
        Self: Sized,
    {
        Ok(Self)
    }

    fn args_are_valid(args: &[&str]) -> bool
    where
        Self: Sized,
    {
        args.is_empty()
    }

    fn execute(
        &self,
        arguments: Vec<String>,
        context: &mut TuiCommandContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if !arguments.is_empty() {
            return Err(format!(
                "The 'close' command does not support arguments: '{}'",
                arguments.join(" ")
            )
            .into());
        }
        context.command_to_execute = Some(AppMessage::Close);
        Ok(())
    }
}
