use crate::redaction_engine::RedactionEngine;
use crate::{Redactor, SimpleRedactor, SimpleRedactorMode};
use data_classification::ClassId;
use std::collections::HashMap;

/// A builder for creating a [`RedactionEngine`].
#[expect(
    missing_debug_implementations,
    reason = "Nothing to output for debugging"
)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_redaction(engine: &RedactionEngine, class_id: &ClassId, input: &str, expected: &str) {
        let mut output = String::new();
        engine.redact_as_class(class_id, input, |s| output.push_str(s));
        assert_eq!(output, expected);
    }

    #[test]
    fn new_creates_builder_with_default_values() {
        let builder = RedactionEngineBuilder::new();
        let engine = builder.build();
        test_redaction(
            &engine,
            &ClassId::new("test_taxonomy", "test_class"),
            "sensitive data",
            "",
        );

        let builder = RedactionEngineBuilder::default();
        let engine = builder.build();
        test_redaction(
            &engine,
            &ClassId::new("test_taxonomy", "test_class"),
            "sensitive data",
            "",
        );
    }

    #[test]
    fn add_multiple_class_redactors() {
        let redactor1 = SimpleRedactor::with_mode(SimpleRedactorMode::Insert("XX".to_string()));
        let redactor2 = SimpleRedactor::with_mode(SimpleRedactorMode::Insert("YY".to_string()));

        let class_id1 = ClassId::new("taxonomy", "class1");
        let class_id2 = ClassId::new("taxonomy", "class2");
        let class_id3 = ClassId::new("taxonomy", "class3");

        let builder = RedactionEngineBuilder::new()
            .add_class_redactor(&class_id1, &redactor1)
            .add_class_redactor(&class_id2, &redactor2);

        let engine = builder.build();
        test_redaction(&engine, &class_id1, "sensitive data", "XX");
        test_redaction(&engine, &class_id2, "sensitive data", "YY");
        test_redaction(&engine, &class_id3, "sensitive data", "");
    }

    #[test]
    fn set_fallback_redactor_overwrites_default() {
        let redactor1 = SimpleRedactor::with_mode(SimpleRedactorMode::Insert("XX".to_string()));
        let redactor2 = SimpleRedactor::with_mode(SimpleRedactorMode::Insert("YY".to_string()));
        let redactor3 = SimpleRedactor::with_mode(SimpleRedactorMode::Insert("ZZ".to_string()));

        let class_id1 = ClassId::new("taxonomy", "class1");
        let class_id2 = ClassId::new("taxonomy", "class2");
        let class_id3 = ClassId::new("taxonomy", "class3");

        let builder = RedactionEngineBuilder::new()
            .add_class_redactor(&class_id1, &redactor1)
            .add_class_redactor(&class_id2, &redactor2)
            .set_fallback_redactor(&redactor3);

        let engine = builder.build();
        test_redaction(&engine, &class_id1, "sensitive data", "XX");
        test_redaction(&engine, &class_id2, "sensitive data", "YY");
        test_redaction(&engine, &class_id3, "sensitive data", "ZZ");
    }
}
