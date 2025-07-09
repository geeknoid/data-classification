use crate as data_classification;
use crate::classified_data_wrapper;

const TAXONOMY: &str = "Default";

#[cfg(feature = "serde")]
classified_data_wrapper!(TAXONOMY, Sensitive, "Holds sensitive data.", Serde);

#[cfg(feature = "serde")]
classified_data_wrapper!(
    TAXONOMY,
    Unknown,
    "Holds data with an unknown classification.",
    Serde
);

#[cfg(feature = "serde")]
classified_data_wrapper!(
    TAXONOMY,
    Unclassified,
    "Holds data which has no classification.",
    Serde
);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(TAXONOMY, Sensitive, "Holds sensitive data.", NoSerde);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(
    TAXONOMY,
    Unknown,
    "Holds data with an unknown classification.",
    NoSerde
);

#[cfg(not(feature = "serde"))]
classified_data_wrapper!(
    TAXONOMY,
    Unclassified,
    "Holda data which has no classification.",
    NoSerde
);
