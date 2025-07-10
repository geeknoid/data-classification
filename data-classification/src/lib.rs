//! This crate provides mechanisms to classify and manipulate sensitive data.
//!
//! Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
//! A user's name, IP address, email address, and other similar information require special treatment. For
//! example, it's usually not legally acceptable to emit a user's email address in a system's logs.
//! Following these rules can be challenging and error-prone, especially when the data is
//! transferred between different components of a large complex system. This crate provides
//! mechanisms to reduce the risk of unintentionally exposing sensitive data.
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
//! This crate is built around two traits:
//!
//! * The [`Classified`] trait is used to mark types that hold sensitive data. The trait exposes
//!   explicit mechanisms to access the data in a safe and auditable way.
//!
//! * The [`Extract`] trait is used to extract sensitive data from a container. It
//!   works a lot like the [`Display`](std::fmt::Display) trait but instead of producing text
//!   intended to be displayed to a user, it produces text intended for redaction.
//!
//! # Classified Data Wrappers
//!
//! A classified data wrapper is used to encapsulate sensitive data. Wrapper types implement both the
//! [`Classified`] and [`Extract`] traits, indicating that they contain sensitive data, which can be
//! extracted for telemetry.
//!
//! The [`classified_data_wrapper!`] macro is the preferred way to define a classified data wrapper type. The macro takes
//! four arguments:
//!
//! - The name of the taxonomy.
//! - The name of the data class.
//! - A comment describing the data class.
//! - A flag indicating whether the data class should support deserialization with serde.
//!
//! Applications use the classified data wrapper types around application
//! data types to indicate instances of those types hold sensitive data. Although applications typically
//! define their own taxonomies of data classes, this crate defines three well-known wrapper types:
//!
//! * [`Sensitive<T>`] which can be used for taxonomy-agnostic classification in libraries.
//! * [`Unknown<T>`] which holds data without a known classification.
//! * [`Unclassified<T>`] which holds data that explicitly has no classification.
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

mod builtin_wrappers;
mod class_id;
mod classified;
mod classified_data_wrapper;
mod extract;
mod extractor;

pub use builtin_wrappers::*;
pub use class_id::ClassId;
pub use classified::Classified;
pub use extract::Extract;
pub use extractor::Extractor;
