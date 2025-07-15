use data_classification::Extract;
use data_redaction::RedactionEngine;
use once_cell::sync::OnceCell;

static REDACTION_ENGINE: OnceCell<RedactionEngine> = OnceCell::new();

pub fn set_redaction_engine_for_logging(engine: RedactionEngine) {
    REDACTION_ENGINE.set(engine).unwrap();
}

pub fn serialize_sensitive(value: &dyn Extract) -> String {
    let engine = REDACTION_ENGINE.get().unwrap();
    let mut output = String::new();
    engine.redact(value, |s| output.push_str(s));
    output
}

macro_rules! log {
    (@fmt ($name:ident) = $value:expr) => {
        format!("{}={}", stringify!($name), $value)
    };

    (@fmt ($name:ident):@ = $value:expr) => {
        format!("{}={}", stringify!($name), crate::logging::serialize_sensitive(&$value))
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
