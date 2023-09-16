//! Quantity values.
//!
//! Following is a list of values sorted in numerical ascending order.
//!
//! * [`ZERO`]
//! * [`FEW`]
//! * [`SOME`]
//! * [`SEVERAL`]
//! * [`HANDFUL`]
//! * [`MANY`]

/// Quantity value for 'zero'.
#[doc = include_str!("doc/const_notes.md")]
pub const ZERO: usize = 0;

/// Quantity value for 'few'.
#[doc = include_str!("doc/const_notes.md")]
pub const FEW: usize = 3;

/// Quantity value for 'some'.
#[doc = include_str!("doc/const_notes.md")]
pub const SOME: usize = 5;

/// Quantity value for 'several'.
#[doc = include_str!("doc/const_notes.md")]
pub const SEVERAL: usize = 7;

/// Quantity value for 'handful'.
#[doc = include_str!("doc/const_notes.md")]
pub const HANDFUL: usize = 10;

/// Quantity value for 'many'.
#[doc = include_str!("doc/const_notes.md")]
pub const MANY: usize = 100;
