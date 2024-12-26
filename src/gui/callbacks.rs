use std::sync::Arc;
use std::sync::Mutex;

use slint::ComponentHandle;

use crate::gui::state::AppState;
use crate::gui::AppWindow;
use crate::gui::Facade;

pub fn register_callbacks(
    ui: &mut AppWindow,
    global_state: Arc<Mutex<AppState>>,
) -> Result<(), crate::gui::error::Error> {
    let facade = ui.global::<Facade>();

    {
        let ui_weak = ui.as_weak();
        let global_state = global_state.clone();
        facade.on_focus_commander(move || {
            tracing::debug!("focus-commander called");
            let ui_weak = ui_weak.unwrap();
            let facade = ui_weak.global::<Facade>();
            let ui_commander_text = facade.get_commander_text();

            global_state
                .lock()
                .unwrap()
                .get_commander_mut()
                .reset(ui_commander_text.into())
                .unwrap();
        });
    }

    {
        let ui_weak = ui.as_weak();
        let global_state = global_state.clone();
        facade.on_unfocus_commander(move || {
            tracing::debug!("unfocus-commander called");
            let _ui_weak = ui_weak.unwrap();

            global_state.lock().unwrap().get_commander_mut().clear();
        });
    }

    {
        let ui_weak = ui.as_weak();
        let global_state = global_state.clone();
        facade.on_commander_keypress(move |key_event| {
            tracing::debug!(?key_event, "commander-keypress called");

            let mut global_state = global_state.lock().unwrap();
            let commander = global_state.get_commander_mut();
            commander.push_keypress(key_event.into()).unwrap();
            let suggestions = commander
                .get_suggestions()
                .into_iter()
                .map(slint::SharedString::from)
                .collect::<Vec<_>>();

            let current_commander_text = commander.get_text();
            drop(global_state);

            let ui_weak = ui_weak.unwrap();
            let facade = ui_weak.global::<Facade>();
            facade.set_commander_text(current_commander_text.into());
            facade
                .set_commander_suggestions(slint::ModelRc::new(slint::VecModel::from(suggestions)));
        });
    }

    Ok(())
}
