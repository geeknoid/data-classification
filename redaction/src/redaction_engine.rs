use crate::Redactor;
use core::fmt::Debug;
use data_classification::{DataClass, Extract, Extractor};
use std::collections::HashMap;

/// Lets you apply redaction to classified data.
///
/// You use [`RedactionEngineBuilder`](crate::RedactionEngineBuilder) to create an instance of this type.
/// The builder lets you configure exactly which redactor to use to redact individual data classes encountered
/// while producing telemetry.
///
/// ## Example
///
/// ```rust
/// use std::fmt::Write;
/// use data_classification::core_taxonomy::{SENSITIVE, Sensitive};
/// use redaction::{SimpleRedactor, SimpleRedactorMode, Redactor, RedactionEngineBuilder};
///
/// struct Person {
///     name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
///     age: u32,
/// }
///
/// fn try_out() {
///     let person = Person {
///         name: "John Doe".to_string().into(),
///         age: 30,
///     };
///
///     let asterisk_redactor = SimpleRedactor::new();
///     let erasing_redactor = SimpleRedactor::with_mode(SimpleRedactorMode::Erase);
///
///     // Create the redaction engine. This is typically done once when the application starts.
///     let engine = RedactionEngineBuilder::new()
///         .add_class_redactor(SENSITIVE, &asterisk_redactor)
///         .set_fallback_redactor(&erasing_redactor)
///         .build();
///
///     let mut output_buffer = String::new();
///
///     engine.redact(&person.name, |s| output_buffer.write_str(s).unwrap());
///
///     // check that the data in the output buffer has indeed been redacted as expected.
///     assert_eq!(output_buffer, "********");
/// }
/// #
/// # fn main() {
/// #     try_out();
/// # }
/// ```
#[derive(Clone)]
pub struct RedactionEngine<'a> {
    redactors: HashMap<DataClass, &'a (dyn Redactor + 'a)>,
    fallback: &'a (dyn Redactor + 'a),
}

impl<'a> RedactionEngine<'a> {
    #[must_use]
    pub(crate) fn new(
        mut redactors: HashMap<DataClass, &'a (dyn Redactor + 'a)>,
        fallback: &'a (dyn Redactor + 'a),
    ) -> Self {
        redactors.shrink_to_fit();

        Self {
            redactors,
            fallback,
        }
    }

    /// Redacts some classified data, sending the results to the output callback.
    pub fn redact(&self, value: &dyn Extract, mut output: impl FnMut(&str)) {
        value.extract(Extractor::new(
            &mut (move |data_class: DataClass, value: &str| {
                self.redact_as_class(data_class, value, &mut output);
            }),
        ));
    }

    /// Redacts a string with an explicit data classification, sending the results to the output callback.
    pub fn redact_as_class(
        &self,
        data_class: DataClass,
        value: impl AsRef<str>,
        mut output: impl FnMut(&str),
    ) {
        let redactor = self.redactors.get(&data_class).unwrap_or(&self.fallback);
        redactor.redact(data_class, value.as_ref(), &mut output);
    }

    /// The exact length of the redacted output if it is a constant.
    ///
    /// This can be used as a hint to optimize buffer allocations.
    #[must_use]
    pub fn exact_len(&self, data_class: DataClass) -> Option<usize> {
        let redactor = self.redactors.get(&data_class).unwrap_or(&self.fallback);
        redactor.exact_len()
    }
}

impl Debug for RedactionEngine<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_list().entries(self.redactors.keys()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{RedactionEngineBuilder, SimpleRedactor, SimpleRedactorMode};
    use core::fmt::Write;
    use data_classification::core_taxonomy::{
        Insensitive, SENSITIVE, Sensitive, UnknownSensitivity,
    };
    use data_classification::data_class;

    const TEST_TAXONOMY: &str = "TestTaxonomy";

    // Define custom wrapper types for testing
    data_class!(TEST_TAXONOMY, Personal, PERSONAL, NoSerde);
    data_class!(TEST_TAXONOMY, Financial, FINANCIAL, NoSerde);

    // Helper function to create a simple test redactor
    fn create_test_redactor(mode: SimpleRedactorMode) -> SimpleRedactor {
        SimpleRedactor::with_mode(mode)
    }

    // Helper function to collect redaction output into a string
    fn collect_output(engine: &RedactionEngine, value: &dyn Extract) -> String {
        let mut output = String::new();
        engine.redact(value, |s| output.push_str(s));
        output
    }

    // Helper function to collect redaction output for explicit class
    fn collect_output_as_class(
        engine: &RedactionEngine,
        data_class: DataClass,
        value: &str,
    ) -> String {
        let mut output = String::new();
        engine.redact_as_class(data_class, value, |s| output.push_str(s));
        output
    }

    #[test]
    fn test_new_creates_engine_with_redactors() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        // Test that the engine was created successfully
        assert_eq!(engine.redactors.len(), 1);
    }

    #[test]
    fn test_redact_uses_specific_redactor_for_registered_class() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let sensitive_data = Sensitive::new("secret".to_string());
        let result = collect_output(&engine, &sensitive_data);

        assert_eq!(result, "******"); // Should be asterisks, not erased
    }

    #[test]
    fn test_redact_uses_fallback_for_unregistered_class() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Replace('X'));

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let unknown_data = UnknownSensitivity::new("john@example.com".to_string());
        let result = collect_output(&engine, &unknown_data);

        assert_eq!(result, "XXXXXXXXXXXXXXXX"); // Should use fallback redactor
    }

    #[test]
    fn test_redact_as_class_uses_specific_redactor() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let result =
            collect_output_as_class(&engine, Sensitive::<()>::data_class(), "confidential");

        assert_eq!(result, "************"); // Should use asterisk redactor
    }

    #[test]
    fn test_redact_as_class_uses_fallback_for_unknown_class() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Replace('?'));

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let unknown_class = DataClass::new("unknown", "test");
        let result = collect_output_as_class(&engine, unknown_class, "data");

        assert_eq!(result, "????"); // Should use fallback redactor
    }

    #[test]
    fn test_redact_with_multiple_redactors() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let hash_redactor = create_test_redactor(SimpleRedactorMode::Replace('#'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );
        _ = redactors.insert(PERSONAL, &hash_redactor as &dyn Redactor);

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let sensitive_data = Sensitive::new("secret".to_string());
        let personal_data = Personal::new("email".to_string());

        let sensitive_result = collect_output(&engine, &sensitive_data);
        let personal_result = collect_output(&engine, &personal_data);

        assert_eq!(sensitive_result, "******");
        assert_eq!(personal_result, "#####");
    }

    #[test]
    fn test_redact_with_different_redactor_modes() {
        let insert_redactor =
            create_test_redactor(SimpleRedactorMode::Insert("[REDACTED]".to_string()));
        let passthrough_redactor = create_test_redactor(SimpleRedactorMode::Passthrough);
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &insert_redactor as &dyn Redactor,
        );
        _ = redactors.insert(
            UnknownSensitivity::<()>::data_class(),
            &passthrough_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let sensitive_data = Sensitive::new("secret".to_string());
        let unknown_data = UnknownSensitivity::new("public".to_string());
        let unclassified_data = Insensitive::new("account123".to_string());

        let sensitive_result = collect_output(&engine, &sensitive_data);
        let unknown_result = collect_output(&engine, &unknown_data);
        let unclassified_result = collect_output(&engine, &unclassified_data);

        assert_eq!(sensitive_result, "[REDACTED]");
        assert_eq!(unknown_result, "public");
        assert_eq!(unclassified_result, ""); // Uses fallback (erase)
    }

    #[test]
    fn test_redact_with_empty_string() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let empty_data = Sensitive::new(String::new());
        let result = collect_output(&engine, &empty_data);

        assert_eq!(result, ""); // Empty string should remain empty
    }

    #[test]
    fn test_redact_as_class_with_empty_string() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let result = collect_output_as_class(&engine, Sensitive::<()>::data_class(), "");

        assert_eq!(result, ""); // Empty string should remain empty
    }

    #[test]
    fn test_engine_clone() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);
        let cloned_engine = engine.clone();

        let sensitive_data = Sensitive::new("test".to_string());
        let original_result = collect_output(&engine, &sensitive_data);
        let cloned_result = collect_output(&cloned_engine, &sensitive_data);

        assert_eq!(original_result, cloned_result);
        assert_eq!(original_result, "****");
    }

    #[test]
    fn test_multiple_output_calls() {
        let passthrough_redactor = create_test_redactor(SimpleRedactorMode::Passthrough);
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &passthrough_redactor as &dyn Redactor,
        );

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        let sensitive_data = Sensitive::new("hello world".to_string());
        let mut call_count = 0;
        let mut total_output = String::new();

        engine.redact(&sensitive_data, |s| {
            call_count += 1;
            total_output.push_str(s);
        });

        assert_eq!(call_count, 1);
        assert_eq!(total_output, "hello world");
    }

    struct Person {
        name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
    }

    #[test]
    fn test_basic() {
        let person = Person {
            name: "John Doe".to_string().into(),
        };

        let asterisk_redactor = SimpleRedactor::new();
        let erasing_redactor = SimpleRedactor::with_mode(SimpleRedactorMode::Erase);

        // Create the redaction engine. This is typically done once when the application starts.
        let engine = RedactionEngineBuilder::new()
            .add_class_redactor(SENSITIVE, &asterisk_redactor)
            .set_fallback_redactor(&erasing_redactor)
            .build();

        let mut output_buffer = String::new();

        engine.redact(&person.name, |s| output_buffer.write_str(s).unwrap());

        assert_eq!(None, engine.exact_len(SENSITIVE));
        assert_eq!(output_buffer, "********");
    }

    #[test]
    fn test_debug_trait_implementation() {
        let asterisk_redactor = create_test_redactor(SimpleRedactorMode::Replace('*'));
        let hash_redactor = create_test_redactor(SimpleRedactorMode::Replace('#'));
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);

        let mut redactors = HashMap::new();
        _ = redactors.insert(
            Sensitive::<()>::data_class(),
            &asterisk_redactor as &dyn Redactor,
        );
        _ = redactors.insert(PERSONAL, &hash_redactor as &dyn Redactor);

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        // Test the Debug trait implementation
        let debug_output = format!("{engine:?}");

        // The Debug implementation should show a list of registered data class keys
        // Since HashMap iteration order is not guaranteed, we need to check that both keys are present
        assert!(debug_output.contains("Sensitive") || debug_output.contains("SENSITIVE"));
        assert!(debug_output.contains("Personal") || debug_output.contains("PERSONAL"));

        // Should be formatted as a debug list (starts with [ and ends with ])
        assert!(debug_output.starts_with('['));
        assert!(debug_output.ends_with(']'));
    }

    #[test]
    fn test_debug_trait_with_empty_redactors() {
        let fallback_redactor = create_test_redactor(SimpleRedactorMode::Erase);
        let redactors = HashMap::new();

        let engine = RedactionEngine::new(redactors, &fallback_redactor);

        // Test the Debug trait implementation with no redactors
        let debug_output = format!("{engine:?}");

        // Should be an empty debug list
        assert_eq!(debug_output, "[]");
    }
}
