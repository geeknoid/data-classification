/// Generates a constant representing a data class, along with a wrapper type to hold data of that class.
///
/// The type produced by this macro is a wrapper around a payload type `T`.
/// The wrapper automatically implements a number of traits to enable the wrapper
/// to be used safely within an application, without the risk of leaking sensitive data.
///
/// For example, if the data held by the wrapper implements the [`Hash`](core::hash::Hash) trait,
/// then so does the wrapper. This makes it possible to put the wrapper instances into a map, just
/// like the original data.
///
/// ## Arguments
///
/// * `taxonomy_name`: The name of the static identifier that holds the name of the taxonomy. The name of a taxonomy is conventionally in `PascalCase`.
/// * `data_class_name`: The name of the data class within the taxonomy. This name is conventionally in `PascalCase`. This name is also used for as the name of the generated wrapper type.
/// * `static_name`: The name of the static constant that will hold the data class definition.
/// * `wrapper_name = wrapper_name`: The name of the wrapper type that will hold data of the data class. This is optional and defaults to the value of `data_class_name`.
/// * `comment` = "comment": A comment describing the data class. This is optional and defaults to an empty string.
/// * `serde`: A flag indicating whether the wrapper should support deserialization with serde.
///   Use `Serde` to enable support and `NoSerde` to skip it.
///
/// ## Example
///
/// ```rust
/// use data_classification::data_class;
///
/// static CONTOSO_TAXONOMY: &str = "Contoso";
///
/// data_class!(CONTOSO_TAXONOMY, CustomerContent, CUSTOMER_CONTENT, Serde);
/// data_class!(CONTOSO_TAXONOMY, CustomerIdentifier, CUSTOMER_IDENTIFIER, Serde);
/// data_class!(CONTOSO_TAXONOMY, OrganizationIdentifier, ORGANIZATION_IDENTIFIER, wrapper_name = FooBar, comment = "An extra comment", Serde);
/// ```
/// The above code generates the following:
///
/// * The static constants `CUSTOMER_CONTENT`, `CUSTOMER_IDENTIFIER`, and `ORGANIZATION_IDENTIFIER` of type `data_classification::DataClass`, which can be used to identify as unique ids for data classes.
/// * The wrapper types `CustomerContent`, `CustomerIdentifier`, and `OrganizationIdentifier` that hold data of the respective data classes.
#[macro_export]
macro_rules! data_class {
    ($taxonomy_name:ident, $data_class_name:ident, $static_name:ident, $serde:tt) => {
        data_class!($taxonomy_name, $data_class_name, $static_name, wrapper_name = $data_class_name, comment = "", $serde);
    };

    ($taxonomy_name:ident, $data_class_name:ident, $static_name:ident, wrapper_name = $wrapper_name:ident, $serde:tt) => {
        data_class!($taxonomy_name, $data_class_name, $static_name, wrapper_name = $wrapper_name, comment = "", $serde);
    };

    ($taxonomy_name:ident, $data_class_name:ident, $static_name:ident, comment = $comment:expr, $serde:tt) => {
        data_class!($taxonomy_name, $data_class_name, $static_name, wrapper_name = $data_class_name, comment = $comment, $serde);
    };

    ($taxonomy_name:ident, $data_class_name:ident, $static_name:ident, comment = $comment:expr, wrapper_name = $wrapper_name:ident, $serde:tt) => {
        data_class!($taxonomy_name, $data_class_name, $static_name, wrapper_name = $wrapper_name, comment = $comment, $serde);
    };

    ($taxonomy_name:ident, $data_class_name:ident, $static_name:ident, wrapper_name = $wrapper_name:ident, comment = $comment:expr, $serde:tt) => {
        #[doc = concat!("Data class definition, part of the [`", stringify!($taxonomy_name), "`] taxonomy.")]
        ///
        #[doc = $comment]
        pub const $static_name: data_classification::DataClass =
            data_classification::DataClass::new($taxonomy_name, stringify!($data_class_name));

        #[doc = concat!("Wrapper holding data of the [`", stringify!($data_class_name), "`] data class in the [`", stringify!($taxonomy_name), "`] taxonomy.")]
        pub struct $wrapper_name<T> {
            payload: T,
        }

        impl<T> $wrapper_name<T> {
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
            pub const fn data_class() -> data_classification::DataClass {
                $static_name
            }
        }

        impl<T> data_classification::Extract for $wrapper_name<T>
        where
            T: core::fmt::Display,
        {
            fn extract(&self, extractor: data_classification::Extractor) {
                extractor.write_str(
                    $static_name,
                    self.payload.to_string().as_str(),
                )
            }
        }

        impl<T> data_classification::Classified<T> for $wrapper_name<T> {
            fn exfiltrate(self) -> T {
                self.payload
            }

            fn visit(&self, operation: impl FnOnce(&T)) {
                operation(&self.payload);
            }

            fn visit_mut(&mut self, operation: impl FnOnce(&mut T)) {
                operation(&mut self.payload);
            }

            fn data_class() -> data_classification::DataClass {
                $static_name
            }
        }

        impl<T> core::fmt::Display for $wrapper_name<T>
        where
            T: core::fmt::Display,
        {
            #[expect(clippy::string_slice, reason = "No problem with UTF-8 here")]
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                static ASTERISKS: &str = "********************************";

                let len = self.payload.to_string().len();
                if len < ASTERISKS.len() {
                    core::write!(f, "{0}<{1}>", stringify!($static_name), &ASTERISKS[0..len])
                } else {
                    core::write!(f, "{0}<{1}>", stringify!($wrapper_name), "*".repeat(len))
                }
            }
        }

        impl<T> core::fmt::Debug for $wrapper_name<T>
        where
            T: core::fmt::Debug,
        {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                core::write!(f, "{}(...)", stringify!($wrapper_name))
            }
        }

        impl<T> core::clone::Clone for $wrapper_name<T>
        where
            T: core::clone::Clone,
        {
            fn clone(&self) -> Self {
                Self {
                    payload: self.payload.clone(),
                }
            }
        }

        impl<T> core::cmp::PartialEq for $wrapper_name<T>
        where
            T: core::cmp::PartialEq,
        {
            fn eq(&self, other: &Self) -> bool {
                self.payload == other.payload
            }
        }

        impl<T> core::cmp::Eq for $wrapper_name<T> where T: core::cmp::Eq {}

        impl<T> core::cmp::PartialOrd for $wrapper_name<T>
        where
            T: core::cmp::PartialOrd,
        {
            fn partial_cmp(&self, other: &Self) -> core::option::Option<core::cmp::Ordering> {
                self.payload.partial_cmp(&other.payload)
            }
        }
        impl<T> core::cmp::Ord for $wrapper_name<T>
        where
            T: core::cmp::Ord,
        {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.payload.cmp(&other.payload)
            }
        }

        impl<T> core::default::Default for $wrapper_name<T>
        where
            T: core::default::Default,
        {
            fn default() -> Self {
                Self {
                    payload: T::default(),
                }
            }
        }

        impl<T> core::hash::Hash for $wrapper_name<T>
        where
            T: core::hash::Hash,
        {
            fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
                self.payload.hash(state);
            }
        }

        impl<T> core::convert::From<T> for $wrapper_name<T> {
            fn from(payload: T) -> Self {
                Self::new(payload)
            }
        }

        data_classification::data_class_deserialize!($wrapper_name, $serde);
        data_classification::data_class_serialize!($wrapper_name, $serde);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! data_class_serialize {
    ($wrapper_name:ident, Serde) => {
        impl<T> serde::Serialize for $wrapper_name<T>
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

    ($wrapper_name:ident, NoSerde) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! data_class_deserialize {
    ($wrapper_name:ident, Serde) => {
        impl<'a, T> serde::Deserialize<'a> for $wrapper_name<T>
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

    ($wrapper_name:ident, NoSerde) => {};
}
