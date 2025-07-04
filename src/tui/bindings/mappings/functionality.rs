
crate::map_key_to_function! {
    name: Abort,
    display: "abort",
    DEFAULT_KEY: crossterm::event::KeyCode::Esc,
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: None,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        tracing::debug!("Deactivating EX");
        app.current_focus = crate::tui::app::FocusState::CommandMode;
        Ok(None)
    }
}
