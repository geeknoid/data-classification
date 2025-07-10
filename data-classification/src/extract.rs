use crate::Extractor;

/// Provides a mechanism to extract sensitive information held by classified data wrapper.
pub trait Extract {
    /// Writes the sensitive information to the extractor.
    fn extract(&self, extractor: Extractor);
}
