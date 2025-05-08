use std::fmt::Display;

/// The identity of a well-known data class.
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct ClassId {
    taxonomy: &'static str,
    class: &'static str,
}

impl ClassId {
    /// Creates a new data class id.
    #[must_use]
    pub const fn new(taxonomy: &'static str, class: &'static str) -> Self {
        Self { taxonomy, class }
    }

    /// Returns the taxonomy of the data class.
    #[must_use]
    pub const fn taxonomy(&self) -> &'static str {
        self.taxonomy
    }

    /// Returns the class name of the data class.
    #[must_use]
    pub const fn class(&self) -> &'static str {
        self.class
    }
}

impl Display for ClassId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.taxonomy, self.class)
    }
}
