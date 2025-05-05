/// Generates a data class type.
///
/// ## Arguments
///
/// * `taxonomy`: The taxonomy to which the data class belongs. This is a string literal that will
///   be used as the return value for the [`Classified::taxonomy`](crate::Classified::taxonomy) method.
/// * `name`: The name of the data class.
/// * `comment`: A comment describing the data class. This will be used as the doc comment for the
///   data class type.
/// * `serde`: A flag indicating whether the data class should support deserialization with serde. Use `Serde` to enable support and `NoSerde` to skip it.
///
/// ## Example
///
/// ```rust
/// use data_classification::data_class;
///
/// data_class!("ContosoTaxonomy", CustomerContent, "Data that represents content produced by a customer", Serde);
/// data_class!("ContosoTaxonomy", CustomerIdentifier, "Data that can identify a customer", Serde);
/// data_class!("ContosoTaxonomy", OrganizationIdentifier, "Data that can identity an organization", Serde);
/// ```
#[macro_export]
macro_rules! data_class {
    ($taxonomy:expr, $name:ident, $comment:expr, $serde:tt) => {
        #[doc = $comment]
        pub struct $name<T> {
            payload: T,
        }

        impl<T> $name<T> {
            /// Creates a new instance of the data class.
            pub const fn new(payload: T) -> Self {
                Self { payload }
            }

            /// Returns the payload of the data class.
            pub fn exfiltrate(self) -> T {
                self.payload
            }

            /// Returns the class of the data which is within the scope of the taxonomy.
            #[must_use]
            pub const fn class() -> &'static str {
                stringify!($name)
            }

            /// Returns the taxonomy of the data class.
            #[must_use]
            pub const fn taxonomy() -> &'static str {
                $taxonomy
            }
        }

        impl<T> data_classification::Classified for $name<T>
        where
            T: std::fmt::Display,
        {
            fn externalize(&self, redactor: data_classification::RedactionSink) {
                redactor.write_str(self.payload.to_string().as_str())
            }

            fn class(&self) -> &'static str {
                stringify!($name)
            }

            fn taxonomy(&self) -> &'static str {
                $taxonomy
            }
        }

        impl<T> data_classification::ClassifiedAccessor<T> for $name<T> {
            fn exfiltrate(self) -> T {
                self.payload
            }

            fn visit(&self, operation: impl FnOnce(&T)) {
                operation(&self.payload);
            }

            fn visit_mut(&mut self, operation: impl FnOnce(&mut T)) {
                operation(&mut self.payload);
            }
        }

        impl<T> std::fmt::Display for $name<T>
        where
            T: std::fmt::Display,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                static ASTERISKS: &str = "********************************";

                let len = self.payload.to_string().len();
                if len < ASTERISKS.len() {
                    std::write!(f, "$name<{0}>", &ASTERISKS[0..len])
                } else {
                    std::write!(f, "$name<{0}>", "*".repeat(len))
                }
            }
        }

        impl<T> std::fmt::Debug for $name<T>
        where
            T: std::fmt::Debug,
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::write!(f, "$name<{self:?}>")
            }
        }

        impl<T> std::clone::Clone for $name<T>
        where
            T: std::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    payload: self.payload.clone(),
                }
            }
        }

        impl<T> std::cmp::PartialEq for $name<T>
        where
            T: std::cmp::PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.payload == other.payload
            }
        }
        impl<T> std::cmp::Eq for $name<T> where T: std::cmp::Eq {}

        impl<T> std::default::Default for $name<T>
        where
            T: std::default::Default,
        {
            fn default() -> Self {
                Self {
                    payload: T::default(),
                }
            }
        }

        impl<T> std::convert::From<T> for $name<T> {
            fn from(payload: T) -> Self {
                Self::new(payload)
            }
        }

        impl<T> std::hash::Hash for $name<T>
        where
            T: std::hash::Hash,
        {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.payload.hash(state);
            }
        }

        data_classification::data_class_deserialize!($serde, $name);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! data_class_deserialize {
    (Serde, $name:ident) => {
        impl<'a, T> serde::Deserialize<'a> for $name<T>
        where
            T: serde::Deserialize<'a>,
        {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'a>,
            {
                let payload = T::deserialize(deserializer)?;
                Ok(Self { payload })
            }
        }
    };

    (NoSerde, $name:ident) => {};
}
