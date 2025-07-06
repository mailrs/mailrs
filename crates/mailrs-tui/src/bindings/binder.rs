use std::collections::HashMap;

use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyModifiers;

use crate::bindings::mappings::KeyToFunctionMapping;
use crate::focus::Focus;

type BindingFn<Context, Err> =
    Box<dyn Fn(&mut Context) -> Result<Option<crate::app::AppMessage>, Err>>;

struct BindingHelper<Context, Err> {
    name: &'static str,
    func: BindingFn<Context, Err>,
}

#[derive(Default)]
pub struct Binder<Context, Err> {
    mapping: HashMap<(KeyCode, KeyModifiers, Focus), BindingHelper<Context, Err>>,
}

impl<State> Binder<State, crate::error::Error> {
    pub fn new() -> Self {
        Self {
            mapping: HashMap::new(),
        }
    }
}

impl<Context, Err> Binder<Context, Err> {
    pub fn with_binding<B>(mut self) -> Self
    where
        B: KeyToFunctionMapping<Context, Error = Err> + 'static,
        Context: 'static,
        Err: 'static,
    {
        fn run_binding<B, Context, Err>(
            app: &mut Context,
        ) -> Result<Option<crate::app::AppMessage>, Err>
        where
            B: KeyToFunctionMapping<Context, Error = Err>,
        {
            B::run(app)
        }

        let keys = (B::DEFAULT_KEY, B::DEFAULT_MODIFIER, B::REQUIRED_FOCUS);

        self.mapping.insert(
            keys,
            BindingHelper {
                name: B::NAME,
                func: Box::new(run_binding::<B, Context, Err>),
            },
        );
        self
    }

    pub fn run_binding_for_keycode(
        &self,
        current_focus: Focus,
        keycode: KeyCode,
        modifiers: KeyModifiers,
        context: &mut Context,
    ) -> Option<Result<Option<crate::app::AppMessage>, Err>> {
        tracing::trace!(
            ?keycode,
            ?modifiers,
            ?current_focus,
            "Trying to find keybinding"
        );

        let Some(helper) = self.mapping.get(&(keycode, modifiers, current_focus)) else {
            tracing::warn!(
                ?keycode,
                ?modifiers,
                ?current_focus,
                "Failed to find keybinding"
            );
            return None;
        };

        tracing::trace!(
            ?keycode,
            ?modifiers,
            ?current_focus,
            name = helper.name,
            "Running binding"
        );
        Some((helper.func)(context))
    }
}
