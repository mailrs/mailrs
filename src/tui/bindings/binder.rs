use std::collections::HashMap;

use crossterm::event::KeyCode;
use crossterm::event::KeyModifiers;

use crate::tui::bindings::mappings::KeyToFunctionMapping;

type BindingFn<Context, Err> = Box<dyn Fn(&mut Context) -> Result<(), Err>>;

struct BindingHelper<Context, Err> {
    name: &'static str,
    func: BindingFn<Context, Err>,
}

pub struct Binder<Context, Err> {
    mapping: HashMap<(KeyCode, KeyModifiers), BindingHelper<Context, Err>>,
}

impl<Context, Err> Binder<Context, Err> {
    #[inline]
    pub fn builder() -> BinderBuilder<Context, Err> {
        BinderBuilder {
            mapping: HashMap::new(),
        }
    }

    pub fn run_binding_for_keycode(
        &self,
        keycode: KeyCode,
        modifiers: KeyModifiers,
        context: &mut Context,
    ) -> Option<Result<(), Err>> {
        let helper = self.mapping.get(&(keycode, modifiers))?;
        Some((helper.func)(context))
    }

    pub fn rebind(
        &mut self,
        old: (KeyCode, KeyModifiers),
        new: (KeyCode, KeyModifiers),
    ) -> Option<()> {
        let binding = self.mapping.remove(&old)?;
        self.mapping.insert(new, binding);
        Some(())
    }

    pub fn rebind_func_by_name(
        &mut self,
        name: &str,
        new_binding: (KeyCode, KeyModifiers),
    ) -> Option<()> {
        let (old_bindings, _helper) = self.find_binding_helper_for_func_name(name)?;
        self.rebind(*old_bindings, new_binding)
    }

    #[allow(clippy::type_complexity)]
    fn find_binding_helper_for_func_name(
        &self,
        name: &str,
    ) -> Option<(&(KeyCode, KeyModifiers), &BindingHelper<Context, Err>)> {
        self.mapping.iter().find(|(_, helper)| helper.name == name)
    }
}

pub struct BinderBuilder<Context, Err> {
    mapping: HashMap<(KeyCode, KeyModifiers), BindingHelper<Context, Err>>,
}

impl<Context, Err> BinderBuilder<Context, Err>
where
    Context: 'static,
{
    pub fn with_binding<B>(mut self) -> Self
    where
        B: KeyToFunctionMapping<Context, Error = Err> + 'static,
        Err: 'static,
    {
        fn run_binding<B, Context, Err>(app: &mut Context) -> Result<(), Err>
        where
            B: KeyToFunctionMapping<Context, Error = Err>,
        {
            B::run(app)
        }

        let keys = (B::DEFAULT_KEY, B::DEFAULT_MODIFIER);

        self.mapping.insert(
            keys,
            BindingHelper {
                name: B::NAME,
                func: Box::new(run_binding::<B, Context, Err>),
            },
        );
        self
    }

    pub fn build(mut self) -> Binder<Context, Err> {
        self.mapping.shrink_to_fit();
        Binder {
            mapping: self.mapping,
        }
    }
}
