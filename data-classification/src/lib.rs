//! This crate provides a way to classify and redact sensitive data.
//!
//! Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
//! A user's name, IP address, email address, and other similar information require special treatment. For
//! example, it's usually not legally acceptable to emit a user's email address in a system's logs.
//! Following these rules can be challenging and error-prone, especially when the data is
//! transferred between different components of a large complex system. This crate provides
//! mechanisms to help reduce the risk of exposing sensitive data.
//!
//! This crate's general model uses wrapping to isolate sensitive data and avoid accidental exposure, and
//! provides redaction mechanisms to enable sensitive data to be used with telemetry safely.
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
//! - **Redaction**: The process of removing or obscuring sensitive information from data.
//!   Redaction is often done by using consistent hashing, replacing the sensitive data with a hash
//!   value that is not reversible. This allows the data to be used for analysis or processing
//!   without exposing the sensitive information.
//!
//! # Traits
//!
//! This crate is built around three traits:
//!
//! * The [`Classified`] trait is used to provide a redacted version of an instance's textual
//!   representation. In other words, it works a lot like the [`Display`](std::fmt::Display) trait,
//!   but instead of producing text intended to be displayed to a user, it produces redacted text
//!   suitable for use in telemetry.
//!
//! * The [`ClassifiedAccessor`] trait is used to mark types that let you access the sensitive
//!   data they hold in a way that can easily be audited.
//!
//! * The [`Redactor`] trait represents types that know how to redact data. Different redactors
//!   do different transformations to the data such as replacing it with asterisks, or replacing it
//!   with a hash value.
//!
//! # Data Classes
//!
//! A data class is a struct wrapper type used to encapsulate sensitive data. Data classes
//! implement both the [`Classified`] and [`ClassifiedAccessor`] traits, indicating that they contain
//! sensitive data, which can be redacted for telemetry.
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
//! define their own taxonomies of data classes, this crate provides the [`Sensitive`] data class,
//! which can be used for taxonomy-agnostic classification in libraries.
//!
//! Data class types hide the data for the value they hold and, as such, they don't implement the
//! [`Display`](std::fmt::Display) trait to prevent accidentally leaking the type's data in a log
//! or other mechanism.
//!
//! # Example
//!
//! ```rust
//! use std::fmt::Write;
//! use data_classification::{AsteriskRedactor, Classified, Redactor, Sensitive};
//!
//! struct Person {
//!     name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
//!     age: u32,
//! }
//!
//! fn try_out() {
//!     use data_classification::RedactionEngineBuilder;
//!     let person = Person {
//!         name: "John Doe".to_string().into(),
//!         age: 30,
//!     };
//!
//!     // Create the redaction engine. This is typically done once when the application starts.
//!     let engine = RedactionEngineBuilder::new()
//!         .set_fallback_redactor(Box::new(AsteriskRedactor::new()))
//!         .build();
//!
//!     let mut output_buffer = String::new();
//!
//!     engine.redact(&person.name, |s| output_buffer.write_str(s).unwrap());
//!
//!     // check that the data in the output buffer has indeed been redacted as expected.
//!     assert_eq!(output_buffer, "********");
//! }
//! #
//! # fn main() {
//! #     try_out();
//! # }
//! ```

mod asterisk_redactor;
mod classified;
mod classified_accessor;
mod data_class;
mod erasing_redactor;
mod nop_redactor;
mod redaction_engine;
mod redaction_engine_builder;
mod redaction_sink;
mod redactor;
mod sensitive;

#[cfg(feature = "xxh3")]
mod xxh3_redactor;

pub use asterisk_redactor::AsteriskRedactor;
pub use classified::Classified;
pub use classified_accessor::ClassifiedAccessor;
pub use erasing_redactor::ErasingRedactor;
pub use nop_redactor::NopRedactor;
pub use redaction_engine::RedactionEngine;
pub use redaction_engine_builder::RedactionEngineBuilder;
pub use redaction_sink::RedactionSink;
pub use redactor::Redactor;
pub use sensitive::Sensitive;

#[cfg(feature = "xxh3")]
pub use crate::xxh3_redactor::xxH3Redactor;
