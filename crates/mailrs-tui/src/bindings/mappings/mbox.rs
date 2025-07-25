use crate::focus::Focus;

crate::map_key_to_function! {
    name: NextMail,
    display: "next_mail",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('j'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Focus next mail");
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            mbox_state.next();
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: PrevMail,
    display: "prev_mail",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('k'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Focus previous mail");
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            mbox_state.prev();
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: NextBox,
    display: "next_box",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('l'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Focus next box");
        app.boxes_state.focus_next();
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: PrevBox,
    display: "prev_box",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('h'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        tracing::debug!("Focus next box");
        app.boxes_state.focus_prev();
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: OpenMessage,
    display: "open_mail",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Enter,
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Box,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            tracing::debug!("Open message");
            mbox_state.show_current_message();
            app.current_focus = Focus::Message;
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: CloseMessage,
    display: "close_mail",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Esc,
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Message,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            tracing::debug!("Close message");
            mbox_state.hide_message();
            app.current_focus = Focus::Box;
        }
        Ok(None)
    }
}
