use core::fmt::Display;
use std::borrow::Cow;

/// The identity of a well-known data class.
///
/// A class id is composed of the name of a data taxonomy and a class name.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ClassId {
    taxonomy: Cow<'static, str>,
    class: Cow<'static, str>,
}

impl ClassId {
    /// Creates a new data class id.
    #[must_use]
    pub const fn new(taxonomy: &'static str, class: &'static str) -> Self {
        Self {
            taxonomy: Cow::Borrowed(taxonomy),
            class: Cow::Borrowed(class),
        }
    }

    /// Returns the taxonomy of the data class.
    #[must_use]
    pub fn taxonomy(&self) -> &str {
        &self.taxonomy
    }

    /// Returns the name of the data class.
    #[must_use]
    pub fn class(&self) -> &str {
        &self.class
    }
}

impl Display for ClassId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.taxonomy, self.class)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn new_should_create_class_id() {
        let class_id = ClassId::new("taxonomy", "class");
        assert_eq!(class_id.taxonomy, "taxonomy");
        assert_eq!(class_id.class, "class");

        assert_eq!(class_id.taxonomy(), "taxonomy");
        assert_eq!(class_id.class(), "class");
    }

    #[test]
    fn display_should_format_correctly() {
        let class_id = ClassId::new("taxonomy", "class");
        assert_eq!(format!("{class_id}"), "taxonomy.class");
    }

    #[test]
    fn derived_traits_should_work_as_expected() {
        let class_id1 = ClassId::new("tax", "class");
        let class_id2 = ClassId::new("tax", "class");
        let class_id3 = ClassId::new("tax", "other");
        let class_id4 = ClassId::new("other_tax", "class");

        // Clone
        assert_eq!(class_id1, class_id1.clone());

        // PartialEq, Eq
        assert_eq!(class_id1, class_id2);
        assert_ne!(class_id1, class_id3);
        assert_ne!(class_id1, class_id4);

        // PartialOrd, Ord
        assert!(class_id1 < class_id3);
        assert!(class_id1 > class_id4);
        assert!(class_id3 > class_id4);
        assert_eq!(class_id1.cmp(&class_id2), core::cmp::Ordering::Equal);

        // Hash
        let mut hasher1 = DefaultHasher::new();
        class_id1.hash(&mut hasher1);
        let hash1 = hasher1.finish();

        let mut hasher2 = DefaultHasher::new();
        class_id2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        let mut hasher3 = DefaultHasher::new();
        class_id3.hash(&mut hasher3);
        let hash3 = hasher3.finish();

        assert_eq!(hash1, hash2);
        assert_ne!(hash1, hash3);
    }

    #[test]
    #[cfg(feature = "serde")]
    fn serde_should_serialize_and_deserialize() {
        let class_id = ClassId::new("taxonomy", "class");
        let serialized = serde_json::to_string(&class_id).unwrap();
        let deserialized: ClassId = serde_json::from_str(&serialized).unwrap();
        assert_eq!(class_id, deserialized);
    }
}
