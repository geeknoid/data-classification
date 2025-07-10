use crate::ClassId;

/// The output sink used to emit sensitive data using the [`crate::Extract`] trait.
#[expect(
    missing_debug_implementations,
    reason = "Nothing to output for debugging"
)]
pub struct Extractor<'a> {
    output: &'a mut dyn FnMut(&ClassId, &str),
}

impl<'a> Extractor<'a> {
    /// Creates a new extractor instance.
    #[must_use]
    pub fn new(output: &'a mut dyn FnMut(&ClassId, &str)) -> Self {
        Self { output }
    }

    /// Where an instance writes sensitive data.
    pub fn write_str(self, id: &ClassId, value: impl AsRef<str>) {
        (self.output)(id, value.as_ref());
    }
}
