/// Represents a classified payload that is sensitive and should be handled with care.
pub trait Classified<T> {
    /// Exfiltrates the payload, allowing it to be used outside the classified context.
    ///
    /// Exfiltration should be done with caution, as it may expose sensitive information.
    ///
    /// # Returns
    /// The original payload.
    fn exfiltrate(self) -> T;

    /// Visits the payload with the provided operation.
    fn visit(&self, operation: impl Fn(&T));    // TODO: FnMut?
    
    /// Returns the name of the class.
    fn class() -> &'static str;

    /// Returns the name of the data classification taxonomy.
    fn taxonomy() -> &'static str;
}
