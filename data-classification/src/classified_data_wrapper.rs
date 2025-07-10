/// Generates a classified data wrapper type.
///
/// The type produced by this macro is a wrapper around a payload type `T`.
/// The wrapper automatically implements a number of traits to enable the wrapper
/// to be used safely within an application, without the risk of leaking sensitive data.
///
/// For example, if the data held by the wrapper implements the [`Hash`](core::hash::Hash) trait,
/// then so does the wrapper. This makes it possible to put the wrapper instances into a map, just
/// like the original data.///
///
/// ## Arguments
///
/// * `taxonomy`: The taxonomy to which the data class belongs. This is a string literal.
/// * `name`: The name of the wrapper.
/// * `comment`: A comment describing the data class. This will be used as the doc comment for the
///   generated wrapper type.
/// * `serde`: A flag indicating whether the wrapper should support deserialization with serde.
///   Use `Serde` to enable support and `NoSerde` to skip it.
///
/// ## Example
///
/// ```rust
/// use data_classification::classified_data_wrapper;
///
/// classified_data_wrapper!("ContosoTaxonomy", CustomerContent, "Data that represents content produced by a customer", Serde);
/// classified_data_wrapper!("ContosoTaxonomy", CustomerIdentifier, "Data that can identify a customer", Serde);
/// classified_data_wrapper!("ContosoTaxonomy", OrganizationIdentifier, "Data that can identity an organization", Serde);
/// ```
#[macro_export]
macro_rules! classified_data_wrapper {
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

            /// Returns the id of the data class.
            #[must_use]
            pub const fn id() -> data_classification::ClassId {
                data_classification::ClassId::new($taxonomy, stringify!($name))
            }
        }

        impl<T> data_classification::Extract for $name<T>
        where
            T: core::fmt::Display,
        {
            fn extract(&self, extractor: data_classification::Extractor) {
                extractor.write_str(
                    &data_classification::ClassId::new($taxonomy, stringify!($name)),
                    self.payload.to_string().as_str(),
                )
            }
        }

        impl<T> data_classification::Classified<T> for $name<T> {
            fn exfiltrate(self) -> T {
                self.payload
            }

            fn visit(&self, operation: impl FnOnce(&T)) {
                operation(&self.payload);
            }

            fn visit_mut(&mut self, operation: impl FnOnce(&mut T)) {
                operation(&mut self.payload);
            }

            fn id() -> data_classification::ClassId {
                data_classification::ClassId::new($taxonomy, stringify!($name))
            }
        }

        impl<T> core::fmt::Display for $name<T>
        where
            T: core::fmt::Display,
        {
            #[expect(clippy::string_slice, reason = "No problem with UTF-8 here")]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                static ASTERISKS: &str = "********************************";

                let len = self.payload.to_string().len();
                if len < ASTERISKS.len() {
                    core::write!(f, "{0}<{1}>", stringify!($name), &ASTERISKS[0..len])
                } else {
                    core::write!(f, "{0}<{1}>", stringify!($name), "*".repeat(len))
                }
            }
        }

        impl<T> core::fmt::Debug for $name<T>
        where
            T: core::fmt::Debug,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::write!(f, "{}(...)", stringify!($name))
            }
        }

        impl<T> core::clone::Clone for $name<T>
        where
            T: core::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    payload: self.payload.clone(),
                }
            }
        }

        impl<T> core::cmp::PartialEq for $name<T>
        where
            T: core::cmp::PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.payload == other.payload
            }
        }

        impl<T> core::cmp::Eq for $name<T> where T: core::cmp::Eq {}

        impl<T> core::cmp::PartialOrd for $name<T>
        where
            T: core::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> core::option::Option<core::cmp::Ordering> {
                self.payload.partial_cmp(&other.payload)
            }
        }
        impl<T> core::cmp::Ord for $name<T>
        where
            T: core::cmp::Ord,
        {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.payload.cmp(&other.payload)
            }
        }

        impl<T> core::default::Default for $name<T>
        where
            T: core::default::Default,
        {
            fn default() -> Self {
                Self {
                    payload: T::default(),
                }
            }
        }

        impl<T> core::hash::Hash for $name<T>
        where
            T: core::hash::Hash,
        {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.payload.hash(state);
            }
        }

        impl<T> core::convert::From<T> for $name<T> {
            fn from(payload: T) -> Self {
                Self::new(payload)
            }
        }

        data_classification::classified_data_wrapper_deserialize!($serde, $name);
        data_classification::classified_data_wrapper_serialize!($serde, $name);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! classified_data_wrapper_serialize {
    (Serde, $name:ident) => {
        impl<T> serde::Serialize for $name<T>
        where
            T: serde::Serialize,
        {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                self.payload.serialize(serializer)
            }
        }
    };

    (NoSerde, $name:ident) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! classified_data_wrapper_deserialize {
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
