use crate as data_classification;
use crate::data_class;

#[cfg(feature = "serde")]
data_class!(
    "Default",
    Sensitive,
    "General-purpose way to mark data as being sensitive",
    Serde
);

#[cfg(not(feature = "serde"))]
data_class!(
    "Default",
    Sensitive,
    "General-purpose way to mark data as being sensitive",
    NoSerde
);
