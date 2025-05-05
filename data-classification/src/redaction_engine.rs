use crate::{Classified, RedactionSink, Redactor};
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq)]
pub struct Key {
    pub taxonomy: &'static str,
    pub class: &'static str,
}

/// Lets you apply redaction to classified data.
pub struct RedactionEngine<'a> {
    redactors: HashMap<Key, &'a (dyn Redactor + 'a)>,
    fallback: &'a (dyn Redactor + 'a),
}

impl<'a> RedactionEngine<'a> {
    #[must_use]
    pub(crate) fn new(
        mut redactors: HashMap<Key, &'a (dyn Redactor + 'a)>,
        fallback: &'a (dyn Redactor + 'a),
    ) -> Self {
        redactors.shrink_to_fit();

        Self {
            redactors,
            fallback,
        }
    }

    /// Redacts some classified data, sending the results to the output callback.
    pub fn redact<F>(&self, value: &dyn Classified, mut output: F)
    where
        F: FnMut(&str),
    {
        let key = Key {
            taxonomy: value.taxonomy(),
            class: value.class(),
        };

        let redactor = self.redactors.get(&key).unwrap_or(&self.fallback);
        let mut cb = move |s: &str| {
            redactor.redact(s, &mut output);
        };

        value.externalize(RedactionSink::new(&mut cb));
    }
}
