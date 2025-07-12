use data_classification::DataClass;

/// Represents types that can redact data.
pub trait Redactor {
    /// Redacts the given value and calls the output function with the redacted value.
    fn redact(&self, data_class: &DataClass, value: &str, output: &mut dyn FnMut(&str));

    /// The exact length of the redacted output if it is a constant.
    ///
    /// This can be used as a hint to optimize buffer allocations.
    #[must_use]
    fn exact_len(&self) -> Option<usize> {
        None
    }
}
