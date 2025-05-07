use crate::Extractor;

/// Provides a mechanism to extract sensitive information held by an instance.
pub trait Extract {
    /// Writes the sensitive information to the extractor.
    fn extract(&self, extractor: Extractor);
}
