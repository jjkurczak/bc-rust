//! Basic utilities for the crypto crates.
//!
//! The functions contained here are not really intended to be used by end users, but you
//! are welcome to do so if you wish.
//! 
//! That said, beware that this crate is not necessarily documented to the same standard as other crates.
//! Since many of the contained helpers are security-critical (such as the constant time module),
//! we will prioritize fixing security bugs over maintaining a stable API for this crate.

#![forbid(unsafe_code)]
#![forbid(missing_docs)]
#![allow(private_bounds)]

pub mod ct;

/// Basic max function. If they are equal, you get back the first one.
pub fn max<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x >= y { x } else { y }
}

/// Basic min function. If they are equal, you get back the first one.
pub fn min<'a, T: PartialOrd>(x: &'a T, y: &'a T) -> &'a T {
    if x <= y { x } else { y }
}
