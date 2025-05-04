use crate::{Classified, RedactionSink, Redactor};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct Key {
    pub taxonomy: &'static str,
    pub class: &'static str,
}

/// Lets you apply redaction to classified data.
pub struct RedactionEngine {
    redactors: HashMap<Key, Box<dyn Redactor>>,
    fallback: Box<dyn Redactor>,
}

impl RedactionEngine {
    #[must_use]
    pub(crate) fn new(
        mut redactors: HashMap<Key, Box<dyn Redactor>>,
        fallback: Box<dyn Redactor>,
    ) -> Self {
        redactors.shrink_to_fit();

        Self {
            redactors,
            fallback,
        }
    }

    /// Redacts some classified data, sending the results to the output callback.
    pub fn redact<F>(&self, value: &dyn Classified, output: F)
    where
        F: FnOnce(&str),
    {
        let key = Key {
            taxonomy: value.taxonomy(),
            class: value.class(),
        };

        let redactor = self.redactors.get(&key).unwrap_or(&self.fallback);
        value.externalize(RedactionSink::new(Box::new(move |s: &str| {
            redactor.redact(s, Box::new(output));
        })));
    }
}
