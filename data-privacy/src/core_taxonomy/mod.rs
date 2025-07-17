//! A simple data taxonomy with universal data classes.

use data_privacy_macros::taxonomy;

/// A simple data taxonomy with universal data classes.
#[cfg_attr(feature = "serde", taxonomy(core, serde = true))]
#[cfg_attr(not(feature = "serde"), taxonomy(core, serde = false))]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum CoreTaxonomy {
    /// General-purpose data class to indicate that data must be treated carefully.
    ///
    /// This data class is typically used in libraries which are agnostic to a
    /// specific data taxonomy.
    Sensitive,

    /// A data class to indicate that data is specifically not classified.
    Insensitive,

    /// A data class to indicate some data has an unknown classification.
    UnknownSensitivity,
}
