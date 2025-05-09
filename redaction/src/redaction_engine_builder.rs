use crate::redaction_engine::RedactionEngine;
use crate::{Redactor, SimpleRedactor, SimpleRedactorMode};
use data_classification::ClassId;
use std::collections::HashMap;

/// A builder for creating a `RedactionEngine`.
pub struct RedactionEngineBuilder<'a> {
    redactors: HashMap<ClassId, &'a (dyn Redactor + 'a)>,
    fallback: &'a (dyn Redactor + 'a),
}

static ERASING_REDACTOR: SimpleRedactor = SimpleRedactor::with_mode(SimpleRedactorMode::Erase);

impl<'a> RedactionEngineBuilder<'a> {
    /// Creates a new instance of `RedactionEngineBuilder`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            redactors: HashMap::new(),
            fallback: &ERASING_REDACTOR,
        }
    }

    /// Adds a redactor for a specific data taxonomy and class.
    #[must_use]
    pub fn add_class_redactor(
        mut self,
        class_id: &ClassId,
        redactor: &'a (dyn Redactor + 'a),
    ) -> Self {
        _ = self.redactors.insert(class_id.clone(), redactor);

        self
    }

    /// Adds a redactor that's a fallback for when there is no redactor registered for a particular
    /// data class.
    ///
    /// The default is to use an `ErasingRedactor`, which simply erases the original string.
    #[must_use]
    pub fn set_fallback_redactor(mut self, redactor: &'a (dyn Redactor + 'a)) -> Self {
        self.fallback = redactor;
        self
    }

    /// Builds the `RedactionEngine`.
    #[must_use]
    pub fn build(self) -> RedactionEngine<'a> {
        RedactionEngine::new(self.redactors, self.fallback)
    }
}

impl Default for RedactionEngineBuilder<'_> {
    fn default() -> Self {
        Self::new()
    }
}
