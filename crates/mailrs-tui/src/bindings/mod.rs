pub mod binder;
pub mod keycode;
pub mod mappings;

#[cfg(test)]
mod tests {
    use crate::bindings::binder::Binder;
    use crate::focus::Focus;

    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    #[cfg_attr(test, derive(PartialEq))]
    pub enum Binding {
        #[serde(rename = "move_left")]
        MoveLeft(super::mappings::movement::MoveLeft),
    }

    #[derive(Debug, serde::Deserialize)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct KeyBinding {
        key: crate::bindings::keycode::KeyCode,
        modifier: Option<crate::bindings::keycode::KeyCode>,
        fun: Binding,
    }

    #[test]
    fn test_deser_bindings() {
        #[derive(Debug, serde::Deserialize)]
        struct KeyBindings {
            bindings: Vec<KeyBinding>,
        }

        let config = r#"
            [[bindings]]
            key = "h"
            fun = "move_left"
        "#;

        let bindings: KeyBindings = toml::from_str(config).unwrap();
        assert_eq!(bindings.bindings.len(), 1);
        assert_eq!(
            bindings.bindings[0],
            KeyBinding {
                key: crate::bindings::keycode::KeyCode::Char('h'),
                modifier: None,
                fun: Binding::MoveLeft(crate::bindings::mappings::movement::MoveLeft),
            },
        )
    }

    #[test]
    fn test_binder() {
        crate::map_key_to_function! {
            name: DummyMoveLeft,
            display: "move_left",
            DEFAULT_KEY: ratatui::crossterm::event::KeyCode::Char('h'),
            DEFAULT_MODIFIER: ratatui::crossterm::event::KeyModifiers::NONE,
            REQUIRED_FOCUS: Focus::Box,
            Error: crate::error::Error,
            context: bool,
            run: |b: &mut bool| {
                *b = true;
                Ok(None)
            }
        }

        let binder = Binder::<bool, crate::error::Error>::new().with_binding::<DummyMoveLeft>();

        let mut context = false;
        let result = binder.run_binding_for_keycode(
            Focus::Box,
            ratatui::crossterm::event::KeyCode::Char('h'),
            ratatui::crossterm::event::KeyModifiers::NONE,
            &mut context,
        );
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        assert!(context, "Binding function did not run");
    }
}
