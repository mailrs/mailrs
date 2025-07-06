pub mod close;
pub mod next_message;
pub mod prev_message;
pub mod query;
pub mod quit;

#[derive(Debug)]
pub struct TuiCommandContext {
    pub command_to_execute: Option<crate::app::AppMessage>,
}

impl tui_commander::Context for TuiCommandContext {}
