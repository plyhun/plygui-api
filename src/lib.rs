#![warn(bare_trait_objects)]
//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names))]
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#![feature(specialization)]

pub mod controls;
pub mod defaults;
pub mod development;
pub mod ids;
pub mod layout;
pub mod types;
pub mod utils;

#[macro_use]
pub mod callbacks;
#[macro_use]
pub mod macros;

#[cfg(feature = "markup")]
pub mod markup;
