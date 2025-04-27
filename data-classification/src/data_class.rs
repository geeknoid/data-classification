/// Generates a data class type.
///
/// ## Arguments
///
/// * `taxonomy`: The taxonomy to which the data class belongs. This is a string literal that will
///   be used as the return value for the [`Classified::taxonomy`](crate::Classified::taxonomy) method.
/// * `name`: The name of the data class.
/// * `comment`: A comment describing the data class. This will be used as the doc comment for the
///   data class type.
/// * `serde`: A boolean indicating whether the data class should support deserialization with serde.
///
/// ## Example
///
/// ```rust
/// use data_classification::data_class;
///
/// data_class!("ContosoTaxonomy", CustomerContent, "Data that represents content produced by a customer", true);
/// data_class!("ContosoTaxonomy", CustomerIdentifier, "Data that can identify a customer", true);
/// data_class!("ContosoTaxonomy", OrganizationIdentifier, "Data that can identity an organization", true);
/// ```
#[macro_export]
macro_rules! data_class {
    ($taxonomy:expr, $name:ident, $comment:expr, $serde:tt) => {
        #[doc = $comment]
        pub struct $name<T> {
            payload: T,
        }
        
        impl<T> $name<T> {
            pub const fn new(payload: T) -> Self {
                Self { payload }
            }

            pub fn exfiltrate(self) -> T {
                self.payload
            }
        }
        
        impl<T> data_classification::Classified<T> for $name<T>
        where
            T: Clone,
        {
            fn exfiltrate(self) -> T {
                self.payload
            }
        
            fn visit(&self, operation: impl Fn(&T)) {
                operation(&self.payload);
            }
            
            fn class() -> &'static str {
                stringify!($name)
            }

            fn taxonomy() -> &'static str {
                $taxonomy
            }
        }
        
        impl<T> data_classification::Redact for $name<T>
        where
            T: std::fmt::Display,
        {
            fn externalize(&self, redactor: &mut data_classification::Redactor) {
                redactor.write_str(self.payload.to_string().as_str())
            }
        }

        impl<T> std::fmt::Display for $name<T>
        where 
            T: std::fmt::Display
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                const ASTERISKS: &str = "********************************";

                let len = self.payload.to_string().len();
                if len < ASTERISKS.len() {
                    std::write!(f, "{0}", &ASTERISKS[0..len])
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
        impl<T> std::cmp::Eq for $name<T>
        where
            T: std::cmp::Eq,
        {
        }
        
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
    (true, $name:ident) => {
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

    (false, $name:ident) => {
    };
}
