use crate::focus::Focus;

crate::map_key_to_function! {
    name: ActivateCommander,
    display: "start_commander",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char(':'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Activating EX");
        app.current_focus = Focus::Commander;
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: DeactivateCommander,
    display: "stop_commander",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Esc,
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::focus::Focus::Commander,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Deactivating EX");
        app.current_focus = Focus::Box;
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: RunCommander,
    display: "commander::run",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Enter,
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::focus::Focus::Commander,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Running commander");

        let mut tui_commander_context =
            crate::commands::TuiCommandContext {
                command_to_execute: None,
            };

        match app.commander.execute(&mut tui_commander_context) {
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

        app.current_focus = Focus::Box;
        app.commander_ui.reset();

        Ok(tui_commander_context.command_to_execute)
    }
}
