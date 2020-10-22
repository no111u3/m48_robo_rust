//! Bit operations and helper traits

use crate::mmio::RegisterLongName;

use core::{
    marker::PhantomData,
    ops::{BitAnd, BitOr, BitOrAssign, Not, Shl, Shr},
};

/// IntLike properties needed to read/write/modify/clear a register.
pub trait IntLike:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitOrAssign
    + Not<Output = Self>
    + Eq
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + Copy
    + Clone
{
    /// Returns zero for every bit
    fn zero() -> Self;

    /// Returns one for every bit
    fn ones() -> Self;
}

impl IntLike for u8 {
    fn zero() -> Self {
        Self::MIN
    }

    fn ones() -> Self {
        Self::MAX
    }
}

impl IntLike for u16 {
    fn zero() -> Self {
        Self::MIN
    }

    fn ones() -> Self {
        Self::MAX
    }
}

impl IntLike for u32 {
    fn zero() -> Self {
        Self::MIN
    }

    fn ones() -> Self {
        Self::MAX
    }
}

impl IntLike for u64 {
    fn zero() -> Self {
        Self::MIN
    }

    fn ones() -> Self {
        Self::MAX
    }
}

/// Specific section of a register.
#[derive(Copy, Clone)]
pub struct Field<T: IntLike, R: RegisterLongName> {
    mask: T,
    pub shift: usize,
    associated_register: PhantomData<R>,
}

/// Values for the specific register fields.
// For the FieldValue, the masks and values are shifted into their actual
// location in the register.
#[derive(Copy, Clone)]
pub struct FieldValue<T: IntLike, R: RegisterLongName> {
    mask: T,
    pub value: T,
    associated_register: PhantomData<R>,
}
