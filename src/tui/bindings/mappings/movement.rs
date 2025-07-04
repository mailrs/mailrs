crate::map_key_to_function! {
    name: MoveLeft,
    display: "move_left",
    DEFAULT_KEY: crossterm::event::KeyCode::Char('h'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::tui::focus::Focus::Box,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        app.boxes_state.focus_prev();
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: MoveDown,
    display: "move_down",
    DEFAULT_KEY: crossterm::event::KeyCode::Char('j'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::tui::focus::Focus::Box,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        let Some(mbox_state) = app.boxes_state.get_current_state_mut() else {
            return Ok(None)
        };

        mbox_state.next();
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: MoveUp,
    display: "move_up",
    DEFAULT_KEY: crossterm::event::KeyCode::Char('k'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::tui::focus::Focus::Box,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        let Some(mbox_state) = app.boxes_state.get_current_state_mut() else {
            return Ok(None)
        };

        mbox_state.prev();
        Ok(None)
    }
}

crate::map_key_to_function! {
    name: MoveRight,
    display: "move_right",
    DEFAULT_KEY: crossterm::event::KeyCode::Char('l'),
    DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
    REQUIRED_FOCUS: crate::tui::focus::Focus::Box,
    Error: crate::tui::error::AppError,
    context: crate::tui::app::AppState,
    run: |app: &mut crate::tui::app::AppState| {
        app.boxes_state.focus_next();
        Ok(None)
    }
}
