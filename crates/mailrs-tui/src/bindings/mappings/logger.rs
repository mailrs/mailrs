crate::map_key_to_function! {
    name: ShowLogger,
    display: "show_logger",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('l'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::CONTROL,
    REQUIRED_FOCUS: crate::focus::Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Toggle logger");
        app.show_logger = !app.show_logger;

        Ok(None)
    }
}
