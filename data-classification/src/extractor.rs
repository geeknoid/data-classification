use crate::DataClass;

/// The output sink used to emit sensitive data using the [`crate::Extract`] trait.
#[expect(
    missing_debug_implementations,
    reason = "Nothing to output for debugging"
)]
pub struct Extractor<'a> {
    output: &'a mut dyn FnMut(&DataClass, &str),
}

impl<'a> Extractor<'a> {
    /// Creates a new extractor instance.
    #[must_use]
    pub fn new(output: &'a mut dyn FnMut(&DataClass, &str)) -> Self {
        Self { output }
    }

    /// Where an instance writes sensitive data.
    pub fn write_str(self, data_class: &DataClass, value: impl AsRef<str>) {
        (self.output)(data_class, value.as_ref());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn write_str_should_call_output_closure() {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let captured_calls_clone = Rc::<RefCell<Vec<(DataClass, String)>>>::clone(&captured_calls);

        let mut output = move |data_class: &DataClass, value: &str| {
            captured_calls_clone
                .borrow_mut()
                .push((data_class.clone(), value.to_string()));
        };

        let extractor = Extractor::new(&mut output);
        let data_class = DataClass::new("test_taxonomy", "test_class");

        extractor.write_str(&data_class, "sensitive_data");

        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, data_class);
        assert_eq!(calls[0].1, "sensitive_data");
    }

    #[test]
    fn write_str_should_work_with_different_data_classes() {
        let captured_calls = Rc::new(RefCell::new(Vec::new()));
        let captured_calls_clone = Rc::<RefCell<Vec<(DataClass, String)>>>::clone(&captured_calls);

        let mut output = move |data_class: &DataClass, value: &str| {
            captured_calls_clone
                .borrow_mut()
                .push((data_class.clone(), value.to_string()));
        };

        let data_class1 = DataClass::new("pii", "email");
        let data_class2 = DataClass::new("financial", "credit_card");

        let extractor1 = Extractor::new(&mut output);
        extractor1.write_str(&data_class1, "user@example.com");

        let extractor2 = Extractor::new(&mut output);
        extractor2.write_str(&data_class2, "1234-5678-9012-3456");

        let calls = captured_calls.borrow();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].0, data_class1);
        assert_eq!(calls[0].1, "user@example.com");
        assert_eq!(calls[1].0, data_class2);
        assert_eq!(calls[1].1, "1234-5678-9012-3456");
    }
}
