//! This crate provides mechanisms to classify and manipulate sensitive data.
//!
//! Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
//! A user's name, IP address, email address, and other similar information require special treatment. For
//! example, it's usually not legally acceptable to emit a user's email address in a system's logs.
//! Following these rules can be challenging and error-prone, especially when the data is
//! transferred between different components of a large complex system. This crate provides
//! mechanisms to reduce the risk of exposing sensitive data.
//!
//! This crate's general model uses wrapping to isolate sensitive data and avoid accidental exposure.
//!
//! # Concepts
//!
//! Before continuing, it's important to understand a few concepts:
//!
//! - **Data Classification**: The process of tagging sensitive data with individual data classes.
//!   Different data classes may have different rules for handling them. For example, some sensitive
//!   data can be put into logs, but only for a limited time, while other data can never be logged.
//!
//! - **Data Taxonomy**: A group of related data classes that together represent a consistent set
//!   of rules for handling sensitive data. Different companies or governments usually have their
//!   own taxonomies.
//!
//! # Traits
//!
//! This crate is built around three traits:
//!
//! * The [`Extract`] trait is used to extract sensitive data from an instance. In other words, it
//!   works a lot like the [`Display`](std::fmt::Display) trait but instead of producing text
//!   intended to be displayed to a user, it produces text intended for redaction.
//!
//! * The [`Classified`] trait is used to mark types that let you access the sensitive
//!   data they hold in a way that can easily be audited.
//!
//! # Data Classes
//!
//! A data class is a struct wrapper type used to encapsulate sensitive data. Data classes
//! implement both the [`Extract`] and [`Classified`] traits, indicating that they contain
//! sensitive data, which can be extracted for telemetry.
//!
//! The [`data_class!`] macro is the preferred way to define a data class type. The macro takes
//! four arguments:
//!
//! - The name of the taxonomy.
//! - The name of the data class.
//! - A comment describing the data class.
//! - A flag indicating whether the data class should support deserialization with serde.
//!
//! The data class type that the macro generates is a wrapper that you use around your own application
//! data types to indicate instances of those types hold sensitive data. Although applications typically
//! define their own taxonomies of data classes, this crate provides the [`Sensitive<T>`] data class,
//! which can be used for taxonomy-agnostic classification in libraries.
//!
//! Data class types hide the data for the value they hold and, as such, they don't implement the
//! [`Display`](std::fmt::Display) trait to prevent accidentally leaking the type's data in a log
//! or other mechanism.
//!
//! # Example
//!
//! ```rust
//! use data_classification::Sensitive;
//!
//! struct Person {
//!     name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
//!     age: u32,
//! }
//!
//! fn try_out() {
//!     let person = Person {
//!         name: "John Doe".to_string().into(),
//!         age: 30,
//!     };
//!
//!     // doesn't compile since `Sensitive` doesn't implement `Display`
//!     // println!("Name: {}", person.name);
//!
//!    // extract the data from the `Sensitive` type
//!    let name = person.name.exfiltrate();
//!    println!("Name: {name}");
//! }
//! #
//! # fn main() {
//! #     try_out();
//! # }
//! ```

mod class_id;
mod classified;
mod data_class;
mod extract;
mod extractor;
mod sensitive;

pub use class_id::ClassId;
pub use classified::Classified;
pub use extract::Extract;
pub use extractor::Extractor;
pub use sensitive::*;
