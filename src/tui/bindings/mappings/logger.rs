crate::map_key_to_function! {
    name: ShowLogger,
    display: "show_logger",
    DEFAULT_KEY: crossterm::event::KeyCode::Char('l'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::CONTROL,
    REQUIRED_FOCUS: None,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        tracing::debug!("Toggle logger");
        app.show_logger = !app.show_logger;

        Ok(None)
    }
}
