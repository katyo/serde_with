#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![cfg_attr(feature = "cargo-clippy", allow(renamed_and_removed_lints))]

//! [![docs.rs badge](https://docs.rs/serde_with/badge.svg)](https://docs.rs/serde_with/)
//! [![crates.io badge](https://img.shields.io/crates/v/serde_with.svg)](https://crates.io/crates/serde_with/)
//! [![Build Status](https://travis-ci.org/jonasbb/serde_with.svg?branch=master)](https://travis-ci.org/jonasbb/serde_with)
//! [![codecov](https://codecov.io/gh/jonasbb/serde_with/branch/master/graph/badge.svg)](https://codecov.io/gh/jonasbb/serde_with)
//!
//! ---
//!
//! This crate provides custom de/serialization helpers to use in combination with [serde's with-annotation][with-annotation].
//!
//! Serde tracks a wishlist of similar helpers at [serde#553].
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies.serde_with]
//! version = "..."
//! features = [ "..." ]
//! ```
//!
//! The crate is divided into different modules.
//! They contain helpers for external crates and must be enabled with the correspondig feature.
//!
//! Annotate your struct or enum to enable the custom de/serializer.
//!
//! ```rust
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde_with;
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(with = "serde_with::rust::display_fromstr")]
//!     bar: u8,
//! }
//! # fn main() {}
//! ```
//!
//! Most helpers implement both deserialize and serialize.
//! If you do not want to derive both, you can simply derive only the necessary parts.
//! If you want to mix different helpers, you can write your annotations like
//!
//! ```rust
//! # extern crate serde;
//! # #[macro_use]
//! # extern crate serde_derive;
//! # extern crate serde_with;
//! # #[cfg(feature = "json")]
//! #[derive(Deserialize, Serialize)]
//! struct Foo {
//!     #[serde(
//!         deserialize_with = "serde_with::rust::display_fromstr::deserialize",
//!         serialize_with = "serde_with::json::nested::serialize"
//!     )]
//!     bar: u8,
//! }
//! # fn main() {}
//! ```
//!
//! However, this will prohibit you from applying deserialize on the value returned by serializing a struct.
//!
//! [with-annotation]: https://serde.rs/field-attrs.html#serdewith--module
//! [serde#553]: https://github.com/serde-rs/serde/issues/553

#[cfg(feature = "chrono")]
extern crate chrono as chrono_crate;
#[doc(hidden)]
#[macro_use]
pub extern crate serde;
#[cfg(feature = "json")]
extern crate serde_json;

#[cfg(feature = "chrono")]
pub mod chrono;
mod duplicate_key_impls;
#[cfg(feature = "json")]
pub mod json;
pub mod rust;
#[doc(hidden)]
pub mod with_prefix;

/// Separator for string-based collection de/serialization
pub trait Separator {
    /// Return the string delimiting two elements in the string-based collection
    fn separator() -> &'static str;
}

/// Predefined separator using a single space
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct SpaceSeparator;

impl Separator for SpaceSeparator {
    #[inline]
    fn separator() -> &'static str {
        " "
    }
}

/// Predefined separator using a single comma
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct CommaSeparator;

impl Separator for CommaSeparator {
    #[inline]
    fn separator() -> &'static str {
        ","
    }
}
