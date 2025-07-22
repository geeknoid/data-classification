//! A simple data taxonomy with universal data classes.

use data_privacy_macros::taxonomy;

/// A simple data taxonomy with universal data classes.
#[cfg_attr(feature = "serde", taxonomy(core, serde = true))]
#[cfg_attr(not(feature = "serde"), taxonomy(core, serde = false))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CoreTaxonomy {
    /// The `sensitive` data class indicates data must be treated carefully.
    ///
    /// This data class is typically used in libraries which are agnostic to a
    /// specific data taxonomy.
    Sensitive,

    /// The `insensitive` data class indicates data is specifically not classified.
    Insensitive,

    /// The `unknown_sensitivity` data class indicates data has an unknown classification.
    UnknownSensitivity,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DataClass;

    #[test]
    fn test_core_taxonomy() {
        assert_eq!(
            CoreTaxonomy::Sensitive.data_class(),
            DataClass::new("core", "sensitive")
        );
        assert_eq!(
            CoreTaxonomy::Insensitive.data_class(),
            DataClass::new("core", "insensitive")
        );
        assert_eq!(
            CoreTaxonomy::UnknownSensitivity.data_class(),
            DataClass::new("core", "unknown_sensitivity")
        );
    }

    #[test]
    fn test_debug_trait() {
        assert_eq!(
            format!("{:?}", Sensitive::new(2)),
            "<core/sensitive:REDACTED>"
        );
        assert_eq!(
            format!("{:?}", Insensitive::new("Hello")),
            "<core/insensitive:REDACTED>"
        );
        assert_eq!(
            format!("{:?}", UnknownSensitivity::new(31.4)),
            "<core/unknown_sensitivity:REDACTED>"
        );
    }
}
