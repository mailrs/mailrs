#[derive(Debug, Default)]
pub struct AppState {
    commander: Commander,
}

impl AppState {
    pub fn get_commander_mut(&mut self) -> &mut Commander {
        &mut self.commander
    }
}

#[derive(Debug, Default)]
pub struct Commander {
    current_text: String,
}

impl Commander {
    pub fn reset(&mut self, text: String) -> Result<(), ()> {
        tracing::debug!(?text, "Resetting commander");
        Ok(())
    }

    pub fn clear(&mut self) {
        tracing::debug!("Clearing commander");
    }

    pub fn push_keypress(&mut self, key_event: String) -> Result<(), ()> {
        tracing::debug!(?key_event, "Received key event in commander");
        self.current_text.push_str(&key_event);
        Ok(())
    }

    pub fn get_suggestions(&self) -> Vec<String> {
        tracing::debug!("Returning suggestions");
        vec![]
    }

    pub(crate) fn get_text(&self) -> String {
        self.current_text.clone()
    }
}
