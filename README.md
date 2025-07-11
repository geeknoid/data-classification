# Data Classification and Redaction

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
* [Classified Containers](#classified-containers)
* [Example](#example)

## Summary

The `data-classification` crate provides mechanisms to classify and manipulate sensitive data. And the companion
`redaction` crate provides mechanisms to redact sensitive data before it is used in telemetry.

Commercial software often needs to handle sensitive data, such as personally identifiable information (PII).
A user's name, IP address, email address, and other similar information require special treatment. For
example, it's usually not legally acceptable to emit a user's email address in a system's logs.
Following these rules can be challenging and error-prone, especially when the data is
transferred between different components of a large complex system. This crate provides
mechanisms to reduce the risk of unintentionally exposing sensitive data.

The `data-classification` crate's general model uses wrapping to isolate sensitive data and avoid accidental exposure,
while the `redaction` crate leverages classification to recognize
sensitive data and provides flexible mechanisms to systematically redact sensitive data
in a variety of ways, ensuring the sensitive data is not leaked in telemetry.

## Concepts

Before continuing, it's important to understand a few concepts:

- **Data Classification**: The process of tagging sensitive data with individual data classes.
  Different data classes may have different rules for handling them. For example, some sensitive
  data can be put into logs, but only for a limited time, while other data can never be logged.

- **Data Taxonomy**: A group of related data classes that together represent a consistent set
  of rules for handling sensitive data. Different companies or governments usually have their
  own taxonomies.

- **Redaction**: The process of removing or obscuring sensitive information from data.
  Redaction is often done by using consistent hashing, replacing the sensitive data with a hash
  value that is not reversible. This allows the data to be used for analysis or processing
  without exposing the sensitive information.

## Traits

These crates are built around three traits:

* The `Classified` trait is used to mark types that hold sensitive data. The trait exposes
  explicit mechanisms to access the data in a safe and auditable way.

* The `Extract` trait is used to extract sensitive data from a container. It
  works a lot like the `Display` trait but instead of producing text
  intended to be displayed to a user, it produces text intended for redaction.

* The `Redactor` trait represents types that know how to redact data. Different redactors
  do different transformations to the data such as replacing it with asterisks or replacing it
  with a hash value.

## Data Classes

A `DataClass` is a struct that represents a single data class within a taxonomy. The struct
contains the name of the taxonomy and the name of the data class.

## Classified Containers

Types that implement the `Classified` trait are said to be classified containers. They encapsulate
an instance of another type. Although containers can be created by hand, they are most commonly created
using the `data_class!` macro. See the documentation for the macro to learn how you define your own
taxonomy and all its data classes.

Applications use the classified container types around application
data types to indicate instances of those types hold sensitive data. Although applications typically
define their own taxonomies of data classes, this crate defines three well-known data classes:

* `Sensitive<T>` which can be used for taxonomy-agnostic classification in libraries.
* `UnknownSensitivity<T>` which holds data without a known classification.
* `Insensitive<T>` which holds data that explicitly has no classification.

## Example

```rust
use data_classification::core_taxonomy::Sensitive;

struct Person {
    name: Sensitive<String>, // a bit of sensitive data we should not leak in logs
    age: u32,
}

fn try_out() {
    let person = Person {
        name: "John Doe".to_string().into(),
        age: 30,
    };

    // doesn't compile since `Sensitive` doesn't implement `Display`
    // println!("Name: {}", person.name);

   // extract the data from the `Sensitive` type
   let name = person.name.exfiltrate();
   println!("Name: {name}");
}
```
