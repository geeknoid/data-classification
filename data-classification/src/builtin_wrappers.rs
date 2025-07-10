use crate as data_classification;
use crate::classified_data_wrapper;

const TAXONOMY: &str = "Default";

#[cfg(feature = "serde")]
classified_data_wrapper!(TAXONOMY, Sensitive, "Holds sensitive data.", Serde);

#[cfg(feature = "serde")]
classified_data_wrapper!(
    TAXONOMY,
    Unknown,
    "Holds data with an unknown classification.",
    Serde
);

#[cfg(feature = "serde")]
classified_data_wrapper!(
    TAXONOMY,
    Unclassified,
    "Holds data which has no classification.",
    Serde
);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(TAXONOMY, Sensitive, "Holds sensitive data.", NoSerde);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(
    TAXONOMY,
    Unknown,
    "Holds data with an unknown classification.",
    NoSerde
);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(
    TAXONOMY,
    Unclassified,
    "Holds data which has no classification.",
    NoSerde
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClassId, Classified};
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    macro_rules! test_wrapper {
        ($wrapper:ident, $module:ident, $expected_name:expr) => {
            mod $module {
                use super::*;
                use crate::$wrapper as Wrapper;

                #[test]
                fn test_new_and_exfiltrate() {
                    let data = "some data".to_string();
                    let wrapped = Wrapper::new(data.clone());
                    assert_eq!(wrapped.exfiltrate(), data);

                    // Test exfiltrate through the Classified trait
                    let wrapped2 = Wrapper::new(data.clone());
                    let trait_exfiltrated =
                        <Wrapper<String> as Classified<String>>::exfiltrate(wrapped2);
                    assert_eq!(trait_exfiltrated, data);

                    // Create new wrappers to test both methods return the same result
                    let wrapped3 = Wrapper::new(data.clone());
                    let wrapped4 = Wrapper::new(data.clone());
                    assert_eq!(
                        wrapped3.exfiltrate(),
                        <Wrapper<String> as Classified<String>>::exfiltrate(wrapped4)
                    );
                }

                #[test]
                fn test_id() {
                    let id = Wrapper::<String>::id();
                    assert_eq!(id, ClassId::new(TAXONOMY, $expected_name));

                    let trait_id = <Wrapper<String> as Classified<String>>::id();
                    assert_eq!(trait_id, ClassId::new(TAXONOMY, $expected_name));

                    // Verify both methods return the same result
                    assert_eq!(id, trait_id);
                }

                #[test]
                fn test_from() {
                    let data = "some data".to_string();
                    let wrapped: Wrapper<_> = data.clone().into();
                    assert_eq!(wrapped.exfiltrate(), data);
                }

                #[test]
                fn test_display() {
                    let data = "secret";
                    let wrapped = Wrapper::new(data);
                    let display = format!("{}", wrapped);
                    assert_eq!(display, format!("{}<******>", stringify!($wrapper)));

                    let data = "secretsecretsecretsecretsecretsecretsecret";
                    let wrapped = Wrapper::new(data);
                    let display = format!("{}", wrapped);
                    assert_eq!(
                        display,
                        format!(
                            "{}<******************************************>",
                            stringify!($wrapper)
                        )
                    );
                }

                #[test]
                fn test_debug() {
                    let data = "secret";
                    let wrapped = Wrapper::new(data);
                    let debug = format!("{:?}", wrapped);
                    assert!(debug.contains(stringify!($wrapper)));
                }

                #[test]
                fn test_clone() {
                    let data = "some data".to_string();
                    let wrapped1 = Wrapper::new(data);
                    let wrapped2 = wrapped1.clone();
                    assert_eq!(wrapped1, wrapped2);
                }

                #[test]
                fn test_partial_eq_and_eq() {
                    let wrapped1 = Wrapper::new(123);
                    let wrapped2 = Wrapper::new(123);
                    let wrapped3 = Wrapper::new(456);
                    assert_eq!(wrapped1, wrapped2);
                    assert_ne!(wrapped1, wrapped3);
                }

                #[test]
                fn test_partial_ord_and_ord() {
                    let wrapped1 = Wrapper::new(1);
                    let wrapped2 = Wrapper::new(2);
                    let wrapped3 = Wrapper::new(1);
                    assert!(wrapped1 < wrapped2);
                    assert!(wrapped2 > wrapped1);
                    assert_eq!(wrapped1.cmp(&wrapped3), core::cmp::Ordering::Equal);
                }

                #[test]
                fn test_default() {
                    let wrapped: Wrapper<String> = Wrapper::default();
                    assert_eq!(wrapped.exfiltrate(), String::default());
                }

                #[test]
                fn test_hash() {
                    let data = "hash me";
                    let wrapped = Wrapper::new(data);

                    let mut hasher1 = DefaultHasher::new();
                    data.hash(&mut hasher1);
                    let hash1 = hasher1.finish();

                    let mut hasher2 = DefaultHasher::new();
                    wrapped.hash(&mut hasher2);
                    let hash2 = hasher2.finish();

                    assert_eq!(hash1, hash2);
                }

                #[test]
                fn test_classified_trait() {
                    let data = 42;
                    let mut wrapped = Wrapper::new(data);

                    // Test id
                    assert_eq!(Wrapper::<i32>::id(), ClassId::new(TAXONOMY, $expected_name));

                    // Test visit
                    let mut visited = false;
                    wrapped.visit(|d| {
                        assert_eq!(*d, data);
                        visited = true;
                    });
                    assert!(visited);

                    // Test visit_mut
                    let mut visited_mut = false;
                    wrapped.visit_mut(|d| {
                        *d = 43;
                        visited_mut = true;
                    });
                    assert!(visited_mut);

                    // Test exfiltrate
                    assert_eq!(wrapped.exfiltrate(), 43);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_serialize() {
                    let data = "test data".to_string();
                    let wrapped = Wrapper::new(data.clone());

                    // Test JSON serialization
                    let json =
                        serde_json::to_string(&wrapped).expect("Failed to serialize to JSON");
                    assert!(json.contains(&data));

                    // Test pretty JSON serialization
                    let pretty_json = serde_json::to_string_pretty(&wrapped)
                        .expect("Failed to serialize to pretty JSON");
                    assert!(pretty_json.contains(&data));

                    // Test value serialization - the wrapper serializes as the payload directly
                    let value =
                        serde_json::to_value(&wrapped).expect("Failed to serialize to Value");
                    assert!(value.is_string());
                    assert_eq!(value.as_str().unwrap(), data);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_deserialize() {
                    let original_data = "test data".to_string();
                    let original_wrapped = Wrapper::new(original_data.clone());

                    // Serialize the wrapped data
                    let json =
                        serde_json::to_string(&original_wrapped).expect("Failed to serialize");

                    // Deserialize back to wrapped type
                    let deserialized: Wrapper<String> =
                        serde_json::from_str(&json).expect("Failed to deserialize");

                    // Verify the deserialized data matches the original
                    assert_eq!(deserialized.exfiltrate(), original_data);

                    // Create a new deserialized instance to test equality
                    let deserialized2: Wrapper<String> =
                        serde_json::from_str(&json).expect("Failed to deserialize");
                    assert_eq!(deserialized2, original_wrapped);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_roundtrip() {
                    // Test with different data types

                    // String data
                    let string_data = "sensitive information".to_string();
                    let string_wrapped = Wrapper::new(string_data.clone());
                    let string_json =
                        serde_json::to_string(&string_wrapped).expect("Failed to serialize string");
                    let string_deserialized: Wrapper<String> =
                        serde_json::from_str(&string_json).expect("Failed to deserialize string");
                    assert_eq!(string_deserialized.exfiltrate(), string_data);

                    // Integer data
                    let int_data = 42i32;
                    let int_wrapped = Wrapper::new(int_data);
                    let int_json =
                        serde_json::to_string(&int_wrapped).expect("Failed to serialize int");
                    let int_deserialized: Wrapper<i32> =
                        serde_json::from_str(&int_json).expect("Failed to deserialize int");
                    assert_eq!(int_deserialized.exfiltrate(), int_data);

                    // Boolean data
                    let bool_data = true;
                    let bool_wrapped = Wrapper::new(bool_data);
                    let bool_json =
                        serde_json::to_string(&bool_wrapped).expect("Failed to serialize bool");
                    let bool_deserialized: Wrapper<bool> =
                        serde_json::from_str(&bool_json).expect("Failed to deserialize bool");
                    assert_eq!(bool_deserialized.exfiltrate(), bool_data);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_deserialize_from_value() {
                    let original_data = vec![1, 2, 3, 4, 5];
                    let original_wrapped = Wrapper::new(original_data.clone());

                    // Serialize to serde_json::Value
                    let value = serde_json::to_value(&original_wrapped)
                        .expect("Failed to serialize to Value");

                    // Deserialize from serde_json::Value
                    let deserialized: Wrapper<Vec<i32>> = serde_json::from_value(value.clone())
                        .expect("Failed to deserialize from Value");

                    // Verify the deserialized data matches the original
                    assert_eq!(deserialized.exfiltrate(), original_data);

                    // Create a new deserialized instance to test equality
                    let deserialized2: Wrapper<Vec<i32>> =
                        serde_json::from_value(value).expect("Failed to deserialize from Value");
                    assert_eq!(deserialized2, original_wrapped);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_deserialize_complex_types() {
                    use std::collections::HashMap;

                    // Test with HashMap
                    let mut map = HashMap::new();
                    _ = map.insert("key1".to_string(), "value1".to_string());
                    _ = map.insert("key2".to_string(), "value2".to_string());

                    let wrapped_map = Wrapper::new(map.clone());
                    let map_json =
                        serde_json::to_string(&wrapped_map).expect("Failed to serialize map");
                    let deserialized_map: Wrapper<HashMap<String, String>> =
                        serde_json::from_str(&map_json).expect("Failed to deserialize map");
                    assert_eq!(deserialized_map.exfiltrate(), map);

                    // Test with nested Vec
                    let nested_vec = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
                    let wrapped_nested = Wrapper::new(nested_vec.clone());
                    let nested_json = serde_json::to_string(&wrapped_nested)
                        .expect("Failed to serialize nested vec");
                    let deserialized_nested: Wrapper<Vec<Vec<i32>>> =
                        serde_json::from_str(&nested_json)
                            .expect("Failed to deserialize nested vec");
                    assert_eq!(deserialized_nested.exfiltrate(), nested_vec);
                }

                #[cfg(feature = "serde")]
                #[test]
                fn test_serde_deserialize_error_handling() {
                    // Test deserialization with invalid JSON
                    let invalid_json = "invalid json";
                    let result: Result<Wrapper<String>, _> = serde_json::from_str(invalid_json);
                    assert!(result.is_err());

                    // Test deserialization with wrong type
                    let string_json = r#""test string""#;
                    let result: Result<Wrapper<i32>, _> = serde_json::from_str(string_json);
                    assert!(result.is_err());
                }
            }
        };
    }

    test_wrapper!(Sensitive, sensitive, "Sensitive");
    test_wrapper!(Unknown, unknown, "Unknown");
    test_wrapper!(Unclassified, unclassified, "Unclassified");
}
