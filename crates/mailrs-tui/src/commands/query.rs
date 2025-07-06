use super::TuiCommandContext;
use crate::app::AppMessage;

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

    fn args_are_valid(args: &[&str]) -> bool
    where
        Self: Sized,
    {
        // Simple sanity check
        tracing::debug!(?args, "Validating arguments");
        !args.is_empty() && !args.iter().all(|s| s.is_empty())
    }

    fn execute(
        &self,
        arguments: Vec<String>,
        context: &mut TuiCommandContext,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        if arguments.is_empty() || arguments.iter().all(|s| s.is_empty()) {
            return Err(Box::new(QueryError::QueryEmpty));
        }
        // TODO: Parse whether arguments are correct notmuch query?
        context.command_to_execute = Some(AppMessage::Query(arguments));
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum QueryError {
    #[error("Query is empty")]
    QueryEmpty,
}
