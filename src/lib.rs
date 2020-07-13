//! A zero-copy MAME ListInfo format DAT files parser and deserializer.
//!
//! ## Usage
//! listinfo-rs provides a lower-level zero-copy expression tree API 
//! as well as a more user friendly Serde deserialization API. 
//!
//! Illustrated here is an example with the expression tree API 
//!
//! ```rust
//! #[test]
//! fn parse_cave_story() {
//!     const CAVE_STORY: &str = r#"clrmamepro (
//!                 name "Cave Story"
//!                 description "Cave Story"
//!                 version 20161204
//!                 comment "libretro | www.libretro.com"
//!             )
//!             game (
//!                 name "Cave Story (En)"
//!                 description "Cave Story (En)"
//!                 developer "Studio Pixel"
//!                 releaseyear "2004"
//!                 rom ( 
//!                     name "Doukutsu.exe"
//!                     size 1478656 
//!                     crc c5a2a3f6 
//!                     md5 38695d3d69d7a0ada8178072dad4c58b 
//!                     sha1 bb2d0441e073da9c584f23c2ad8c7ab8aac293bf
//!                 )
//!             )
//!         "#;
//!
//!     let document = parse::parse_document(CAVE_STORY).unwrap();
//!     let header = document.entry("clrmamepro").unwrap().next().unwrap();
//!     let game = document.entry("game").unwrap().next().unwrap();
//!     let rom = game.entry_unique("rom").unwrap();
//!     assert_eq!(
//!         header.entry_unique("name"),
//!         Some(&EntryData::Scalar("Cave Story"))
//!     );
//!     assert_eq!(
//!         game.entry_unique("name"),
//!         Some(&EntryData::Scalar("Cave Story (En)"))
//!     );
//!     assert_eq!(
//!         header.entry_unique("name"),
//!         Some(&EntryData::Scalar("Cave Story"))
//!     );
//!     if let EntryData::SubEntry(rom) = rom {
//!         assert_eq!(rom.value_unique("name"), Some("Doukutsu.exe"))
//!     }
//! }
//! ```
//! 
//! ## Features
//! listinfo-rs supports the following features
//!  * `std` Enables `std` support (enabled by default)
//!  * `deserialize` Enables support for serde deserialization
//!
//! ## `no_std`
//! listinfo-rs requires `alloc`, but otherwise is fully supported on `#![no_std]`
//! environments.
//! 
//! You can enable `no_std` support like in Cargo.toml
//!
//! ```toml
//! listinfo = { version = "0.3", default-features = false }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std as alloc;

mod elements;
mod error;

pub mod iter;

#[cfg(test)]
mod tests;

pub mod parse;

#[cfg(feature="deserialize")]
pub mod de;

pub use error::*;
pub use elements::*;
