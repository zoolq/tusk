#![allow(clippy::tabs_in_doc_comments)]
#![deny(clippy::missing_const_for_fn)]
#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

/// The core memory units.
pub mod units;

/// Memory constants for conversion between units.
pub mod constants;

/// Macros for easy creation of memory units.
#[doc(hidden)]
#[cfg(feature = "macro")]
pub mod macros;

#[doc(hidden)]
mod bytes_impl;

/// Trait for converting between memory units.
pub trait MemoryUnit {}
