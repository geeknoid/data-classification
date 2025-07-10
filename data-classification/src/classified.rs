use crate::ClassId;

/// Represents a container that holds classified state.
///
/// Types that implement this trait are containers of classified data. They hide an
/// instance they are given to ensure it is handled carefully throughout the application.
/// Although instances are encapsulated, it's possible to extract the instances when
/// classification is no longer needed.
pub trait Classified<T> {
    /// Exfiltrates the payload, allowing it to be used outside the classified context.
    ///
    /// Exfiltration should be done with caution, as it may expose sensitive information.
    ///
    /// # Returns
    /// The original payload.
    #[must_use]
    fn exfiltrate(self) -> T;

    /// Visits the payload with the provided operation.
    fn visit(&self, operation: impl FnOnce(&T));

    /// Visits the payload with the provided operation.
    fn visit_mut(&mut self, operation: impl FnOnce(&mut T));

    /// Returns the id of the data class of the classified data.
    #[must_use]
    fn id() -> ClassId;
}
