use core::fmt::Display;
use data_privacy::{Classified, RedactionEngine};
use once_cell::sync::OnceCell;

static REDACTION_ENGINE: OnceCell<RedactionEngine> = OnceCell::new();

pub fn set_redaction_engine_for_logging(engine: RedactionEngine) {
    REDACTION_ENGINE.set(engine).unwrap();
}

pub struct Wrapper<'a, C, T> {
    value: &'a C,
    _marker: core::marker::PhantomData<T>,
}

impl<'a, C, T> Wrapper<'a, C, T> {
    pub const fn new(value: &'a C) -> Self {
        Self {
            value,
            _marker: core::marker::PhantomData,
        }
    }
}

impl<C, T> Display for Wrapper<'_, C, T>
where
    C: Classified<T>,
    T: Display,
{
    #[expect(
        clippy::unwrap_in_result,
        reason = "This is a demo app, so we expect the redaction engine to be set up correctly."
    )]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let engine = REDACTION_ENGINE.get().unwrap();
        engine.display_redacted(self.value, |s| {
            _ = f.write_str(s);
        });
        Ok(())
    }
}

macro_rules! log {
    (@fmt ($name:ident) = $value:expr) => {
        format!("{}={}", stringify!($name), $value)
    };

    (@fmt ($name:ident):? = $value:expr) => {
        format!("{}={:?}", stringify!($name), $value)
    };

    (@fmt ($name:ident):@ = $value:expr) => {
        format!("{}={}", stringify!($name), crate::logging::Wrapper::new(&$value))
    };

    ($($name:ident $(: $kind:tt)? = $value:expr),* $(,)?) => {
        let mut parts: Vec<String> = Vec::new();
        $(
            parts.push(log!(@fmt ($name)$(: $kind)? = $value));
        )*
        println!("LOG RECORD: {}", parts.join(", "));
    };
}

pub(crate) use log;
