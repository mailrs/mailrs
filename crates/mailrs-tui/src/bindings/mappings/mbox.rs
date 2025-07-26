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
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('q'),
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

crate::map_key_to_function! {
    name: ScrollMessageDown,
    display: "scroll_message_down",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('j'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Message,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            if let Some(message_state) = mbox_state.currently_shown_message_mut() {
                tracing::debug!("Scrolling down");
                message_state.body_state.scroll_down();
            } else {
                tracing::warn!("No messsage shown currently");
            }
        } else {
            tracing::warn!("No mbox found");
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: ScrollMessageUp,
    display: "scroll_message_up",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('k'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Message,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            if let Some(message_state) = mbox_state.currently_shown_message_mut() {
                tracing::debug!("Scrolling up");
                message_state.body_state.scroll_up();
            } else {
                tracing::warn!("No messsage shown currently");
            }
        } else {
            tracing::warn!("No mbox found");
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: ScrollMessageTop,
    display: "scroll_message_top",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('g'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: Focus::Message,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            if let Some(message_state) = mbox_state.currently_shown_message_mut() {
                tracing::debug!("Scrolling to top");
                message_state.body_state.scroll_to_top();
            } else {
                tracing::warn!("No messsage shown currently");
            }
        } else {
            tracing::warn!("No mbox found");
        }
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: ScrollMessageBottom,
    display: "scroll_message_bottom",
    DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('G'),
    DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::SHIFT,
    REQUIRED_FOCUS: Focus::Message,
    Error: crate::error::Error,
    context: crate::app::AppState,
    run: |app: &mut crate::app::AppState| {
        if let Some(mbox_state) = app.boxes_state.get_current_state_mut() {
            if let Some(message_state) = mbox_state.currently_shown_message_mut() {
                tracing::debug!("Scrolling to bottom");
                message_state.body_state.scroll_to_bottom();
            } else {
                tracing::warn!("No messsage shown currently");
            }
        } else {
            tracing::warn!("No mbox found");
        }
        Ok(None)
    }
}
