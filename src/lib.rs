#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

mod elements;
mod error;
#[cfg(test)]
mod tests;

/// ListInfo parsing functions.
pub mod parse;

pub use error::*;
pub use elements::*;

