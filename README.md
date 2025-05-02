# Data Classification

[![crate.io](https://img.shields.io/crates/v/data-classification.svg)](https://crates.io/crates/data)
[![docs.rs](https://docs.rs/data-classification/badge.svg)](https://docs.rs/data-classification)
[![CI](https://github.com/geeknoid/data-classification/workflows/main/badge.svg)](https://github.com/geeknoid/data-classification/actions)
[![Coverage](https://codecov.io/gh/geeknoid/data-classification/graph/badge.svg?token=FCUG0EL5TI)](https://codecov.io/gh/geeknoid/data-classification)
[![Minimum Supported Rust Version 1.85](https://img.shields.io/badge/MSRV-1.85-blue.svg)]()
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

* [Summary](#summary)
* [Concepts](#concepts)
* [Traits](#traits)
* [Data Classes](#data-classes)
* [Example](#example)

## Summary

This crate provides a way to classify and redact sensitive data.

Commercial cloud software often needs to handle sensitive data, such as personally identifiable information (PII).
A user's name, IP address, email address, and other similar information require special treatment. For
example, it's usually not legally acceptable to emit a user's email address in a system's logs.
Following these rules can be challenging and error-prone, especially when the data is
transferred between different components of a large complex system. This crate provides
mechanisms to help reduce the risk of exposing sensitive data.

This crate's general model uses wrapping to isolate sensitive data and avoid accidental exposure, and
provides redaction mechanisms to enable sensitive data to be used with telemetry safely.

## Concepts

Before continuing, it's important to understand a few concepts:

- **Data Classification**: The process of tagging sensitive data with individual data classes.
  Different data classes may have different rules for handling them. For example, some sensitive
  data can be put into logs, but only for a limited time, while other data can never be logged.

- **Data Taxonomy**: A group of related data classes that together represent a consistent set
  of rules for handling sensitive data. Different companies or governments usually have their own taxonomies.

- **Redaction**: The process of removing or obscuring sensitive information from data.
  Redaction is often done by using consistent hashing, replacing the sensitive data with a hash
  value that is not reversible. This allows the data to be used for analysis or processing
  without exposing the sensitive information.

## Traits

This crate is built around three traits:

* The `Classified` trait is used to mark types that contain sensitive data. The trait lets you
  query the name of the data class and lets you access the sensitive data in a controlled
  and auditable way.

* The `Redact` trait is used to provide a redacted version of an instance's textual representation.
  In other words, it works a lot like the `Display` trait, but instead of producing text intended
  to be displayed to a user, it produces redacted text suitable for use in telemetry.

* The `RedactorMaker` trait is used to create redactors. A type that implements this trait
  is typically initialized when an application starts and is then handed to the telemetry system.
  The telemetry system then creates redactors as needed when it needs to manipulate
  sensitive data.

## Data Classes

A data class is a struct wrapper type used to encapsulate sensitive data. Data classes
implement both the `Classified` and `Redact` traits, indicating that they contain sensitive
data, which can be redacted for telemetry.

The `data_class!` macro is the preferred way to define a data class type. The macro takes
four arguments:

- The name of the taxonomy.
- The name of the data class.
- A comment describing the data class.
- A boolean indicating whether the data class should support deserialization with serde.

The data class type that the macro generates is a wrapper that you use around your own application
data types to indicate instances of those types hold sensitive data. Although applications typically
define their own taxonomies of data classes, this crate provides the `Sensitive` data class,
which can be used for taxonomy-agnostic classification in libraries.

Data class types hide the data for the value they hold and, as such, they don't implement the
`Display` trait to prevent accidentally leaking the type's data in a log or other mechanism.

## Example

```rust
use std::fmt::Write;
use data_classification::{AsteriskRedactorMaker, Redact, RedactorMaker, Sensitive};

struct Person {
    name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
    age: u32,
}

fn try_out() {
    let person = Person {
        name: "John Doe".to_string().into(),
        age: 30,
    };

    // Create the redactor maker. This is typically done once when the application starts.
    // There are different redactor makers available which each produce redactors that
    // redact data in specific ways.
    let redactor_maker = AsteriskRedactorMaker::new();

    let mut output_buffer = String::new();

    {
        // Create a redactor. This will usually be done within the telemetry system when it needs to
        // extract some sensitive data for use in logs.
        let mut redactor = redactor_maker.make_redactor(|s| output_buffer.write_str(s).unwrap());

        // extract the name, redacting it in the process
        person.name.externalize(&mut redactor);
    }

    // check that the data in the output buffer has indeed been redacted as expected.
    assert_eq!(output_buffer, "********");
}
```
