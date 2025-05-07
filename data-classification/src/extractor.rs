use crate::ClassId;

/// The output sink used to emit sensitive data from an instance.
pub struct Extractor<'a> {
    output: &'a mut dyn FnMut(&ClassId, &str),
}

impl<'a> Extractor<'a> {
    /// Creates a new extractor instance.
    #[must_use]
    pub fn new(output: &'a mut dyn FnMut(&ClassId, &str)) -> Self {
        Self { output }
    }

    /// Where to write sensitive data.
    pub fn write_str(self, id: &ClassId, value: &str) {
        (self.output)(id, value);
    }
}
