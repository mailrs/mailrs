crate::map_key_to_function! {
    name: ActivateCommander,
    display: "start_commander",
    DEFAULT_KEY: crossterm::event::KeyCode::Char(':'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: None,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        tracing::debug!("Activating EX");
        app.current_focus = Some(crate::tui::bindings::focus::Focus::Commander);
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: DeactivateCommander,
    display: "stop_commander",
    DEFAULT_KEY: crossterm::event::KeyCode::Esc,
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Some(crate::tui::bindings::focus::Focus::Commander),
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        tracing::debug!("Deactivating EX");
        app.current_focus = None;
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: RunCommander,
    display: "commander::run",
    DEFAULT_KEY: crossterm::event::KeyCode::Enter,
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Some(crate::tui::bindings::focus::Focus::Commander),
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        tracing::debug!("Running commander");

        let mut tui_commander_context =
            crate::tui::commands::TuiCommandContext {
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

        app.current_focus = None;
        app.commander_ui.reset();

        Ok(tui_commander_context.command_to_execute)
    }
}
