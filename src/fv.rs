//! Frequency values.
//!
//! Following is a list of values sorted in numerical ascending order.
//!
//! * [`NEVER`]
//! * [`ALMOST_NEVER`]
//! * [`RARELY`]
//! * [`OCCASIONALLY`]
//! * [`SOMETIMES`]
//! * [`OFTEN`]
//! * [`ALMOST`]
//! * [`ALWAYS`]

/// Frequency value for 'never'.
#[doc = include_str!("doc/const_notes.md")]
pub const NEVER: f64 = 0.0;

/// Frequency value for 'almost never'.
#[doc = include_str!("doc/const_notes.md")]
pub const ALMOST_NEVER: f64 = 0.03;

/// Frequency value for 'rarely'.
#[doc = include_str!("doc/const_notes.md")]
pub const RARELY: f64 = 0.1;

/// Frequency value for 'occasionally'.
#[doc = include_str!("doc/const_notes.md")]
pub const OCCASIONALLY: f64 = 0.3;

/// Frequency value for 'sometimes'.
#[doc = include_str!("doc/const_notes.md")]
pub const SOMETIMES: f64 = 0.5;

/// Frequency value for 'often'.
#[doc = include_str!("doc/const_notes.md")]
pub const OFTEN: f64 = 0.7;

/// Frequency value for 'almost'.
#[doc = include_str!("doc/const_notes.md")]
pub const ALMOST: f64 = 0.9;

/// Frequency value for 'always'.
#[doc = include_str!("doc/const_notes.md")]
pub const ALWAYS: f64 = 1.0;
