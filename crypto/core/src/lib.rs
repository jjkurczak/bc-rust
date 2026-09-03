//! This crate defines the core traits and types used by the rest of the bc-rust.test library.

#![cfg_attr(not(feature = "std"), no_std)]

#![forbid(unsafe_code)]
#![forbid(missing_docs)]

pub mod errors;
pub mod key_material;
pub mod suspendable_state;
pub mod traits;
