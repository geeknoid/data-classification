use crate::data_class;
use crate as data_classification;

#[cfg(feature = "serde")]
data_class!("Default", Sensitive, "General-purpose way to mark data as being sensitive", true);

#[cfg(not(feature = "serde"))]
data_class!("Default", Sensitive, "General-purpose way to mark data as being sensitive", false);
