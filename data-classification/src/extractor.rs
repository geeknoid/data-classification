use crate::DataClass;

/// The output sink used to emit sensitive data using the [`crate::Extract`] trait.
#[expect(
    missing_debug_implementations,
    reason = "Nothing to output for debugging"
)]
pub struct Extractor<'a> {
    output: &'a mut dyn FnMut(DataClass, &str),
}

impl<'a> Extractor<'a> {
    /// Creates a new extractor instance.
    #[must_use]
    pub fn new(output: &'a mut dyn FnMut(DataClass, &str)) -> Self {
        Self { output }
    }

    /// Where an instance writes sensitive data.
    pub fn write_str(self, data_class: DataClass, value: impl AsRef<str>) {
        (self.output)(data_class, value.as_ref());
    }
}
