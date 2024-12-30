pub mod binder;
pub mod keycode;
pub mod mappings;

#[cfg(test)]
mod tests {
    use crate::tui::bindings::binder::Binder;
    use crate::tui::bindings::mappings::map_key_to_function;
    use crate::tui::bindings::mappings::MoveLeft;

    #[derive(Debug, serde::Deserialize)]
    #[serde(untagged)]
    #[cfg_attr(test, derive(PartialEq))]
    pub enum Binding {
        #[serde(rename = "move_left")]
        MoveLeft(MoveLeft),
    }

    #[derive(Debug, serde::Deserialize)]
    #[cfg_attr(test, derive(PartialEq))]
    pub struct KeyBinding {
        key: crate::tui::bindings::keycode::KeyCode,
        modifier: Option<crate::tui::bindings::keycode::KeyCode>,
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
                key: crate::tui::bindings::keycode::KeyCode::Char('h'),
                modifier: None,
                fun: Binding::MoveLeft(MoveLeft),
            },
        )
    }

    #[test]
    fn test_binder() {
        map_key_to_function! {
            name: DummyMoveLeft,
            display: "move_left",
            DEFAULT_KEY: crossterm::event::KeyCode::Char('h'),
            DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
            Error: crate::tui::error::AppError,
            context: bool,
            run: |b: &mut bool| {
                *b = true;
                Ok(())
            }
        }

        let binder = Binder::builder().with_binding::<DummyMoveLeft>().build();

        let mut context = false;
        let result = binder.run_binding_for_keycode(
            crossterm::event::KeyCode::Char('h'),
            crossterm::event::KeyModifiers::NONE,
            &mut context,
        );
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        assert!(context, "Binding function did not run");
    }

    #[test]
    fn test_binder_rebind() {
        map_key_to_function! {
            name: DummyMoveLeft,
            display: "move_left",
            DEFAULT_KEY: crossterm::event::KeyCode::Char('h'),
            DEFAULT_MODIFIER: crossterm::event::KeyModifiers::NONE,
            Error: crate::tui::error::AppError,
            context: bool,
            run: |b: &mut bool| {
                *b = true;
                Ok(())
            }
        }

        let mut binder = Binder::builder().with_binding::<DummyMoveLeft>().build();
        let rebind_res = binder.rebind_func_by_name(
            "move_left",
            (
                crossterm::event::KeyCode::Char('l'),
                crossterm::event::KeyModifiers::NONE,
            ),
        );
        assert!(rebind_res.is_some());

        let mut context = false;
        let result = binder.run_binding_for_keycode(
            crossterm::event::KeyCode::Char('h'),
            crossterm::event::KeyModifiers::NONE,
            &mut context,
        );
        assert!(result.is_none());
        assert!(
            !context,
            "Context changed but function should not have been run"
        );

        let result = binder.run_binding_for_keycode(
            crossterm::event::KeyCode::Char('l'),
            crossterm::event::KeyModifiers::NONE,
            &mut context,
        );
        assert!(result.is_some());
        let result = result.unwrap();
        assert!(result.is_ok());
        assert!(context, "Binding function did not run");
    }
}
