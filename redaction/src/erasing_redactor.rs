use crate::Redactor;
use data_classification::ClassId;

/// Produces redactors that simply erase the original string.
#[derive(Clone)]
pub struct ErasingRedactor {}

impl ErasingRedactor {
    /// Creates a new instance.
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }
}

impl Redactor for ErasingRedactor {
    fn redact<'a>(&self, _: &ClassId, _: &str, _: &'a mut dyn FnMut(&str)) {
        // nothing
    }

    fn exact_len(&self) -> Option<usize> {
        Some(0)
    }
}

impl Default for ErasingRedactor {
    fn default() -> Self {
        Self::new()
    }
}
