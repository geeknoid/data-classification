use crate::DataClass;

/// Represents a container that holds classified state.
///
/// Types that implement this trait are containers of classified data. They hide an
/// instance they are given to ensure it is handled carefully throughout the application.
/// Although instances are encapsulated, it's possible to extract the instances when
/// classification is no longer needed.
///
/// # Example
///
/// ```rust
/// use data_privacy::{Classified, DataClass};
///
/// struct Person {
///    name: String,
///    address: String,
/// }
///
/// impl Person {
///     fn new(name: String, address: String) -> Self {
///         Self { name, address }
///     }
/// }
///
/// struct ClassifiedPerson {
///     person: Person
/// }
///
/// impl ClassifiedPerson {
///    fn new(person: Person) -> Self {
///        Self { person }
///    }
/// }
///
/// impl Classified<Person> for ClassifiedPerson {
///     fn declassify(self) -> Person {
///         self.person
///     }
///
///     fn visit(&self, operation: impl FnOnce(&Person)) {
///         operation(&self.person);
///     }
///
///     fn visit_mut(&mut self, operation: impl FnOnce(&mut Person)) {
///         operation(&mut self.person);
///     }
///
///     fn data_class(&self) -> DataClass {
///         DataClass::new("example_taxonomy", "classified_person")
///     }
/// }
///  ```
pub trait Classified<T> {
    /// Exfiltrates the payload, allowing it to be used outside the classified context.
    ///
    /// Exfiltration should be done with caution, as it may expose sensitive information.
    ///
    /// # Returns
    /// The original payload.
    #[must_use]
    fn declassify(self) -> T;

    /// Visits the payload with the provided operation.
    fn visit(&self, operation: impl FnOnce(&T));

    /// Visits the payload with the provided operation.
    fn visit_mut(&mut self, operation: impl FnOnce(&mut T));

    /// Returns the data class of the classified data.
    #[must_use]
    fn data_class(&self) -> DataClass;
}
