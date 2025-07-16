//! Mechanisms to redact sensitive data before it is used in telemetry.
//!
//! Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
//! A user's name, IP address, email address, and other similar information require special treatment. For
//! example, it's usually not legally acceptable to emit a user's email address in a system's logs.
//! Following these rules can be challenging and error-prone, especially when the data is
//! transferred between different components of a large complex system. This crate provides
//! mechanisms to reduce the risk of unintentionally exposing sensitive data.
//!
//! This crate leverages the `data-classification` crate's data classification model to recognize
//! sensitive data and provide flexible mechanisms to systematically redact such data
//! in a variety of ways, ensuring the sensitive data is not leaked in telemetry.
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
//! # Example
//!
//! ```rust
//! use std::fmt::Write;
//! use data_classification::core_taxonomy::{CoreTaxonomy, Sensitive};
//! use data_redaction::{SimpleRedactor, SimpleRedactorMode, Redactor, RedactionEngineBuilder};
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
//!     let asterisk_redactor = SimpleRedactor::new();
//!     let erasing_redactor = SimpleRedactor::with_mode(SimpleRedactorMode::Erase);
//!
//!     // Create the redaction engine. This is typically done once when the application starts.
//!     let engine = RedactionEngineBuilder::new()
//!         .add_class_redactor(&CoreTaxonomy::Sensitive.data_class(), asterisk_redactor)
//!         .set_fallback_redactor(erasing_redactor)
//!         .build();
//!
//!     let mut output_buffer = String::new();
//!
//!     // Redact the sensitive data in the person's name using the redaction engine.
//!     engine.display_redacted(&person.name, |s| output_buffer.write_str(s).unwrap());
//!
//!     // check that the data in the output buffer has indeed been redacted as expected.
//!     assert_eq!(output_buffer, "********");
//! }
//! #
//! # fn main() {
//! #     try_out();
//! # }
//! ```

mod redaction_engine;
mod redaction_engine_builder;
mod redactor;
mod simple_redactor;

#[cfg(feature = "xxh3")]
mod xxh3_redactor;

pub use redaction_engine::RedactionEngine;
pub use redaction_engine_builder::RedactionEngineBuilder;
pub use redactor::Redactor;
pub use simple_redactor::{SimpleRedactor, SimpleRedactorMode};

#[cfg(feature = "xxh3")]
pub use crate::xxh3_redactor::xxH3Redactor;
