mod functionality;
mod logger;
mod movement;
pub use self::functionality::*;
pub use self::logger::*;
pub use self::movement::*;

pub trait KeyToFunctionMapping<Context> {
    const DEFAULT_KEY: crossterm::event::KeyCode;
    const DEFAULT_MODIFIER: crossterm::event::KeyModifiers;
    const NAME: &'static str;

    type Error;

    fn run(app: &mut Context) -> Result<(), Self::Error>;
}

#[derive(Debug, thiserror::Error)]
#[error("Input '{}' does not match '{}'", .got, .expected)]
pub struct DoesNotMatch {
    pub expected: String,
    pub got: String,
}

#[macro_export]
macro_rules! map_key_to_function {
    (
        name: $name:ident,
        display: $display:literal,
        DEFAULT_KEY: $key:expr,
        DEFAULT_MODIFIER: $modif:expr,
        Error: $errty:ty,
        context: $context:ty,
        run: $fun:expr
    ) => {
        #[derive(Debug)]
        #[cfg_attr(test, derive(PartialEq))]
        pub struct $name;
        impl $crate::tui::bindings::mappings::KeyToFunctionMapping<$context> for $name {
            const DEFAULT_KEY: crossterm::event::KeyCode = $key;
            const DEFAULT_MODIFIER: crossterm::event::KeyModifiers = $modif;
            const NAME: &'static str = $display;
            type Error = $errty;

            fn run(app: &mut $context) -> Result<(), Self::Error> {
                $fun(app)
            }
        }

        impl std::str::FromStr for $name {
            type Err = $crate::tui::bindings::mappings::DoesNotMatch;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if value == $display {
                    Ok(Self)
                } else {
                    Err($crate::tui::bindings::mappings::DoesNotMatch {
                        expected: $display.to_string(),
                        got: value.to_string(),
                    })
                }
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                let s = String::deserialize(deserializer)?;
                std::str::FromStr::from_str(&s).map_err(serde::de::Error::custom)
            }
        }
    };
}
