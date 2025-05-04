/// Provides access to the sensitive information held by an instance.
pub trait ClassifiedAccessor<T> {
    /// Exfiltrates the payload, allowing it to be used outside the classified context.
    ///
    /// Exfiltration should be done with caution, as it may expose sensitive information.
    ///
    /// # Returns
    /// The original payload.
    fn exfiltrate(self) -> T;

    /// Visits the payload with the provided operation.
    fn visit(&self, operation: impl FnOnce(&T));

    /// Visits the payload with the provided operation.
    fn visit_mut(&mut self, operation: impl FnOnce(&mut T));
}
