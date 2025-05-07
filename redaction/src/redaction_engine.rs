use crate::Redactor;
use data_classification::{ClassId, Extract, Extractor};
use std::collections::HashMap;

/// Lets you apply redaction to classified data.
#[derive(Clone)]
pub struct RedactionEngine<'a> {
    redactors: HashMap<ClassId, &'a (dyn Redactor + 'a)>,
    fallback: &'a (dyn Redactor + 'a),
}

impl<'a> RedactionEngine<'a> {
    #[must_use]
    pub(crate) fn new(
        mut redactors: HashMap<ClassId, &'a (dyn Redactor + 'a)>,
        fallback: &'a (dyn Redactor + 'a),
    ) -> Self {
        redactors.shrink_to_fit();

        Self {
            redactors,
            fallback,
        }
    }

    /// Redacts some classified data, sending the results to the output callback.
    pub fn redact<F>(&self, value: &dyn Extract, mut output: F)
    where
        F: FnMut(&str),
    {
        let mut cb = move |class_id: &ClassId, v: &str| {
            let redactor = self.redactors.get(class_id).unwrap_or(&self.fallback);
            redactor.redact(class_id, v, &mut output);
        };

        value.extract(Extractor::new(&mut cb));
    }
}
