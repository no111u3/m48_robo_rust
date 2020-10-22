//! Memory Mapped Input/Output types and functions.

pub mod bitopts;

pub mod registers;

pub mod static_ref;

/// Descriptive name for each register.
pub trait RegisterLongName {}

impl RegisterLongName for () {}

/// Conversion of raw register value into enumerated values member.
/// Implemented inside register_bitfields! macro for each bit field.
pub trait TryFromValue<V> {
    type EnumType;

    fn try_from(v: V) -> Option<Self::EnumType>;
}
