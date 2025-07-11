//! Mechanisms to classify and manipulate sensitive data.
//!
//! Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
//! A user's name, IP address, email address, and other similar information require special treatment. For
//! example, it's usually not legally acceptable to emit a user's email address in a system's logs.
//! Following these rules can be challenging and error-prone, especially when the data is
//! transferred between different components of a large complex system. This crate provides
//! mechanisms to reduce the risk of unintentionally exposing sensitive data.
//!
//! This general model uses wrapping to isolate sensitive data and avoid accidental exposure.
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
//!   works a lot like the [`Display`](std::fmt::Display) trait, but instead of producing text
//!   intended to be displayed to a user, it produces text intended for redaction.
//!
//! # Data Classes
//!
//! A [`DataClass`] is a struct that represents a single data class within a taxonomy. The struct
//! contains the name of the taxonomy and the name of the data class.
//!
//! # Classified Containers
//!
//! Types that implement the [`Classified`] trait are said to be classified containers. They encapsulate
//! an instance of another type. Although containers can be created by hand, they are most commonly created
//! using the [`data_class!`] macro. See the documentation for the macro to learn how you define your own
//! taxonomy and all its data classes.
//!
//! Applications use the classified container types around application
//! data types to indicate instances of those types hold sensitive data. Although applications typically
//! define their own taxonomies of data classes, this crate defines three well-known data classes:
//!
//! * [`Sensitive<T>`](core_taxonomy::Sensitive) which can be used for taxonomy-agnostic classification in libraries.
//! * [`UnknownSensitivity<T>`](core_taxonomy::UnknownSensitivity) which holds data without a known classification.
//! * [`Insensitive<T>`](core_taxonomy::Insensitive) which holds data that explicitly has no classification.
//!
//! # Example
//!
//! ```rust
//! use data_classification::core_taxonomy::Sensitive;
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

mod classified;
pub mod core_taxonomy;
mod data_class_macro;
mod data_class_struct;
mod extract;
mod extractor;

pub use classified::Classified;
pub use data_class_struct::DataClass;
pub use extract::Extract;
pub use extractor::Extractor;
