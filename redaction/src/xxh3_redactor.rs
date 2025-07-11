use crate::Redactor;
use data_classification::DataClass;
use xxhash_rust::xxh3::xxh3_64_with_secret;

const DEFAULT_SECRET_SIZE: usize = 192;
const DEFAULT_SECRET: [u8; DEFAULT_SECRET_SIZE] = [
    0xb8, 0xfe, 0x6c, 0x39, 0x23, 0xa4, 0x4b, 0xbe, 0x7c, 0x01, 0x81, 0x2c, 0xf7, 0x21, 0xad, 0x1c,
    0xde, 0xd4, 0x6d, 0xe9, 0x83, 0x90, 0x97, 0xdb, 0x72, 0x40, 0xa4, 0xa4, 0xb7, 0xb3, 0x67, 0x1f,
    0xcb, 0x79, 0xe6, 0x4e, 0xcc, 0xc0, 0xe5, 0x78, 0x82, 0x5a, 0xd0, 0x7d, 0xcc, 0xff, 0x72, 0x21,
    0xb8, 0x08, 0x46, 0x74, 0xf7, 0x43, 0x24, 0x8e, 0xe0, 0x35, 0x90, 0xe6, 0x81, 0x3a, 0x26, 0x4c,
    0x3c, 0x28, 0x52, 0xbb, 0x91, 0xc3, 0x00, 0xcb, 0x88, 0xd0, 0x65, 0x8b, 0x1b, 0x53, 0x2e, 0xa3,
    0x71, 0x64, 0x48, 0x97, 0xa2, 0x0d, 0xf9, 0x4e, 0x38, 0x19, 0xef, 0x46, 0xa9, 0xde, 0xac, 0xd8,
    0xa8, 0xfa, 0x76, 0x3f, 0xe3, 0x9c, 0x34, 0x3f, 0xf9, 0xdc, 0xbb, 0xc7, 0xc7, 0x0b, 0x4f, 0x1d,
    0x8a, 0x51, 0xe0, 0x4b, 0xcd, 0xb4, 0x59, 0x31, 0xc8, 0x9f, 0x7e, 0xc9, 0xd9, 0x78, 0x73, 0x64,
    0xea, 0xc5, 0xac, 0x83, 0x34, 0xd3, 0xeb, 0xc3, 0xc5, 0x81, 0xa0, 0xff, 0xfa, 0x13, 0x63, 0xeb,
    0x17, 0x0d, 0xdd, 0x51, 0xb7, 0xf0, 0xda, 0x49, 0xd3, 0x16, 0x55, 0x26, 0x29, 0xd4, 0x68, 0x9e,
    0x2b, 0x16, 0xbe, 0x58, 0x7d, 0x47, 0xa1, 0xfc, 0x8f, 0xf8, 0xb8, 0xd1, 0x7a, 0xd0, 0x31, 0xce,
    0x45, 0xcb, 0x3a, 0x8f, 0x95, 0x16, 0x04, 0x28, 0xaf, 0xd7, 0xfb, 0xca, 0xbb, 0x4b, 0x40, 0x7e,
];

const REDACTED_LEN: usize = 16;

/// A redactor that replaces the original string with the xxH3 hash of the string.
#[expect(
    non_camel_case_types,
    reason = "Just following the naming conventions of xxHash, silly as they are"
)]
#[derive(Clone, Debug)]
pub struct xxH3Redactor {
    secret: Box<[u8]>,
}

impl xxH3Redactor {
    /// Creates a new instance.
    #[must_use]
    pub fn new() -> Self {
        Self {
            secret: Box::from(DEFAULT_SECRET),
        }
    }

    /// Creates a new instance with a custom secret.
    ///
    /// The secret must be at least 16 bytes long and at most 256 bytes long.
    #[must_use]
    pub fn with_secret(secret: impl AsRef<[u8]>) -> Self {
        Self {
            secret: Box::from(secret.as_ref()),
        }
    }
}

impl Redactor for xxH3Redactor {
    fn redact(&self, _: DataClass, value: &str, output: &mut dyn FnMut(&str)) {
        let hash = xxh3_64_with_secret(value.as_bytes(), &self.secret);
        let buffer = u64_to_hex_array(hash);

        // SAFETY: The buffer is guaranteed to be valid UTF-8 because it only contains hex digits.
        output(unsafe { core::str::from_utf8_unchecked(&buffer) });
    }

    fn exact_len(&self) -> Option<usize> {
        Some(REDACTED_LEN)
    }
}

impl Default for xxH3Redactor {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn u64_to_hex_array(mut value: u64) -> [u8; 16] {
    static HEX_LOWER_CHARS: &[u8; 16] = b"0123456789abcdef";

    let mut buffer = [0u8; REDACTED_LEN];
    for e in buffer.iter_mut().rev() {
        *e = HEX_LOWER_CHARS[(value & 0x0f) as usize];
        value >>= 4;
    }

    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_redactor_with_default_secret() {
        let redactor = xxH3Redactor::new();
        assert_eq!(redactor.secret.len(), DEFAULT_SECRET_SIZE);
        assert_eq!(redactor.secret.as_ref(), &DEFAULT_SECRET);
    }

    #[test]
    fn test_with_secret_creates_redactor_with_custom_secret() {
        let custom_secret = b"custom_secret_for_testing_purposes";
        let redactor = xxH3Redactor::with_secret(custom_secret);
        assert_eq!(redactor.secret.as_ref(), custom_secret);
    }

    #[test]
    fn test_default_trait_implementation() {
        let redactor = xxH3Redactor::default();
        assert_eq!(redactor.secret.len(), DEFAULT_SECRET_SIZE);
        assert_eq!(redactor.secret.as_ref(), &DEFAULT_SECRET);
    }

    #[test]
    fn test_exact_len_returns_correct_length() {
        let redactor = xxH3Redactor::new();
        assert_eq!(redactor.exact_len(), Some(REDACTED_LEN));
    }

    #[test]
    fn test_redact_produces_consistent_output() {
        let redactor = xxH3Redactor::new();
        let data_class = DataClass::new("test_taxonomy", "test_class");
        let input = "sensitive_data";

        let mut output1 = String::new();
        let mut output2 = String::new();

        redactor.redact(data_class, input, &mut |s| output1.push_str(s));
        redactor.redact(data_class, input, &mut |s| output2.push_str(s));

        assert_eq!(output1, output2);
        assert_eq!(output1.len(), REDACTED_LEN);
    }

    #[test]
    fn test_redact_output_is_hex_string() {
        let redactor = xxH3Redactor::new();
        let data_class = DataClass::new("test_taxonomy", "test_class");
        let input = "test_input";

        let mut output = String::new();
        redactor.redact(data_class, input, &mut |s| output.push_str(s));

        assert_eq!(output.len(), REDACTED_LEN);
        assert!(output.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(
            output
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        );
    }

    #[test]
    fn test_different_inputs_produce_different_outputs() {
        let redactor = xxH3Redactor::new();
        let data_class = DataClass::new("test_taxonomy", "test_class");

        let mut output1 = String::new();
        let mut output2 = String::new();

        redactor.redact(data_class, "input1", &mut |s| output1.push_str(s));
        redactor.redact(data_class, "input2", &mut |s| output2.push_str(s));

        assert_ne!(output1, output2);
    }

    #[test]
    fn test_different_secrets_produce_different_outputs() {
        let redactor1 = xxH3Redactor::new();
        // Create a custom secret that's at least 136 bytes (xxHash minimum)
        let custom_secret = vec![0x42u8; 136];
        let redactor2 = xxH3Redactor::with_secret(&custom_secret);
        let data_class = DataClass::new("test_taxonomy", "test_class");
        let input = "same_input";

        let mut output1 = String::new();
        let mut output2 = String::new();

        redactor1.redact(data_class, input, &mut |s| output1.push_str(s));
        redactor2.redact(data_class, input, &mut |s| output2.push_str(s));

        assert_ne!(output1, output2);
    }

    #[test]
    fn test_empty_string_input() {
        let redactor = xxH3Redactor::new();
        let data_class = DataClass::new("test_taxonomy", "test_class");

        let mut output = String::new();
        redactor.redact(data_class, "", &mut |s| output.push_str(s));

        assert_eq!(output.len(), REDACTED_LEN);
        assert!(output.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_unicode_input() {
        let redactor = xxH3Redactor::new();
        let data_class = DataClass::new("test_taxonomy", "test_class");
        let input = "こんにちは世界"; // "Hello World" in Japanese

        let mut output = String::new();
        redactor.redact(data_class, input, &mut |s| output.push_str(s));

        assert_eq!(output.len(), REDACTED_LEN);
        assert!(output.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_u64_to_hex_array() {
        let result = u64_to_hex_array(0x1234_5678_9abc_def0);
        let expected = b"123456789abcdef0";
        assert_eq!(result, *expected);

        let result = u64_to_hex_array(0);
        let expected = b"0000000000000000";
        assert_eq!(result, *expected);

        let result = u64_to_hex_array(u64::MAX);
        let expected = b"ffffffffffffffff";
        assert_eq!(result, *expected);
    }

    #[test]
    fn test_clone_produces_identical_redactor() {
        // Create a custom secret that's at least 136 bytes (xxHash minimum)
        let custom_secret = vec![0x33u8; 136];
        let original = xxH3Redactor::with_secret(&custom_secret);
        let cloned = original.clone();

        assert_eq!(original.secret, cloned.secret);

        let data_class = DataClass::new("test_taxonomy", "test_class");
        let input = "test_input";

        let mut output1 = String::new();
        let mut output2 = String::new();

        original.redact(data_class, input, &mut |s| output1.push_str(s));
        cloned.redact(data_class, input, &mut |s| output2.push_str(s));

        assert_eq!(output1, output2);
    }

    #[test]
    fn test_custom_secret_edge_cases() {
        // Test with minimum viable secret (136 bytes for xxHash)
        let small_secret = vec![0x11u8; 136];
        let redactor = xxH3Redactor::with_secret(&small_secret);
        assert_eq!(redactor.secret.len(), 136);

        // Test with larger secret
        let large_secret = vec![0u8; 256];
        let redactor = xxH3Redactor::with_secret(&large_secret);
        assert_eq!(redactor.secret.len(), 256);
    }

    #[test]
    fn test_data_class_does_not_affect_output() {
        let redactor = xxH3Redactor::new();
        let data_class1 = DataClass::new("test_taxonomy", "class1");
        let data_class2 = DataClass::new("test_taxonomy", "class2");
        let input = "test_input";

        let mut output1 = String::new();
        let mut output2 = String::new();

        redactor.redact(data_class1, input, &mut |s| output1.push_str(s));
        redactor.redact(data_class2, input, &mut |s| output2.push_str(s));

        // The data_class parameter is ignored in the redaction process
        assert_eq!(output1, output2);
    }
}
