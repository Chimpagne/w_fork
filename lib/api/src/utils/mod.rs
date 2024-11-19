//! Useful data types, functions and traits used throughout the crate to interact with WebAssembly
//! entities such as [`Memory`] and [`Value`].

/// Convert bynary data into [`bytes::Bytes`].
mod into_bytes;
pub use into_bytes::IntoBytes;

/// Useful data types, functions and traits for the interaction between host types and WebAssembly.
pub(crate) mod native;
pub use native::*;

/// Useful data types, functions and traits for interacting with the memory of a [`crate::Instance`].
pub(crate) mod mem;
pub use mem::*;

/// Useful macros to generate enums to represent `Runtime`-types.
pub(crate) mod rt_macros;

pub(crate) mod polyfill;
