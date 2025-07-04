use std::collections::HashMap;

use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

use crate::tui::bindings::mappings::KeyToFunctionMapping;
use crate::tui::focus::Focus;

type BindingFn<Context, Err> =
    Box<dyn Fn(&mut Context) -> Result<Option<crate::tui::app::AppMessage>, Err>>;

struct BindingHelper<Context, Err> {
    name: &'static str,
    func: BindingFn<Context, Err>,
}

#[derive(Default)]
pub struct Binder<Context, Err> {
    mapping: HashMap<(KeyCode, KeyModifiers, Option<Focus>), BindingHelper<Context, Err>>,
}

impl Binder<crate::tui::app::AppState, crate::tui::error::AppError> {
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
        ) -> Result<Option<crate::tui::app::AppMessage>, Err>
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
        current_focus: Option<Focus>,
        keycode: KeyCode,
        modifiers: KeyModifiers,
        context: &mut Context,
    ) -> Option<Result<Option<crate::tui::app::AppMessage>, Err>> {
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

    pub fn rebind(
        &mut self,
        focus: Option<Focus>,
        old: (KeyCode, KeyModifiers),
        new: (KeyCode, KeyModifiers),
    ) -> Option<()> {
        let binding = self.mapping.remove(&(old.0, old.1, focus))?;
        self.mapping.insert((new.0, new.1, focus), binding);
        Some(())
    }

    pub fn rebind_func_by_name(
        &mut self,
        name: &str,
        new_binding: (KeyCode, KeyModifiers),
    ) -> Option<()> {
        let ((keycode, modifiers, focus), _helper) =
            self.find_binding_helper_for_func_name(name)?;
        self.rebind(*focus, (*keycode, *modifiers), new_binding)
    }

    #[allow(clippy::type_complexity)]
    fn find_binding_helper_for_func_name(
        &self,
        name: &str,
    ) -> Option<(
        &(KeyCode, KeyModifiers, Option<Focus>),
        &BindingHelper<Context, Err>,
    )> {
        self.mapping.iter().find(|(_, helper)| helper.name == name)
    }
}
