//! Bit operations and helper traits

use crate::mmio::{RegisterLongName, TryFromValue};

use core::{
    marker::PhantomData,
    ops::{Add, AddAssign, BitAnd, BitOr, BitOrAssign, Not, Shl, Shr},
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

impl<T: IntLike, R: RegisterLongName> Field<T, R> {
    #[inline]
    pub fn read(self, val: T) -> T {
        (val & (self.mask << self.shift)) >> self.shift
    }

    #[inline]
    /// Check if one or more bits in a field are set
    pub fn is_set(self, val: T) -> bool {
        val & (self.mask << self.shift) != T::zero()
    }

    #[inline]
    /// Read value of the field as an enum member
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(self, val: T) -> Option<E> {
        E::try_from(self.read(val))
    }
}

// For the Field, the mask is unshifted, ie. the LSB should always be set
impl<R: RegisterLongName> Field<u8, R> {
    pub const fn new(mask: u8, shift: usize) -> Field<u8, R> {
        Field {
            mask: mask,
            shift: shift,
            associated_register: PhantomData,
        }
    }

    pub fn val(&self, value: u8) -> FieldValue<u8, R> {
        FieldValue::<u8, R>::new(self.mask, self.shift, value)
    }
}

impl<R: RegisterLongName> Field<u16, R> {
    pub const fn new(mask: u16, shift: usize) -> Field<u16, R> {
        Field {
            mask: mask,
            shift: shift,
            associated_register: PhantomData,
        }
    }

    pub fn val(&self, value: u16) -> FieldValue<u16, R> {
        FieldValue::<u16, R>::new(self.mask, self.shift, value)
    }
}

impl<R: RegisterLongName> Field<u32, R> {
    pub const fn new(mask: u32, shift: usize) -> Field<u32, R> {
        Field {
            mask: mask,
            shift: shift,
            associated_register: PhantomData,
        }
    }

    pub fn val(&self, value: u32) -> FieldValue<u32, R> {
        FieldValue::<u32, R>::new(self.mask, self.shift, value)
    }
}

impl<R: RegisterLongName> Field<u64, R> {
    pub const fn new(mask: u64, shift: usize) -> Field<u64, R> {
        Field {
            mask: mask,
            shift: shift,
            associated_register: PhantomData,
        }
    }

    pub fn val(&self, value: u64) -> FieldValue<u64, R> {
        FieldValue::<u64, R>::new(self.mask, self.shift, value)
    }
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

// Necessary to split the implementation of new() out because the bitwise
// math isn't treated as const when the type is generic.
// Tracking issue: https://github.com/rust-lang/rfcs/pull/2632
impl<R: RegisterLongName> FieldValue<u8, R> {
    pub const fn new(mask: u8, shift: usize, value: u8) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value & mask) << shift,
            associated_register: PhantomData,
        }
    }
}

impl<R: RegisterLongName> From<FieldValue<u8, R>> for u8 {
    fn from(val: FieldValue<u8, R>) -> u8 {
        val.value
    }
}

impl<R: RegisterLongName> FieldValue<u16, R> {
    pub const fn new(mask: u16, shift: usize, value: u16) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value & mask) << shift,
            associated_register: PhantomData,
        }
    }
}

impl<R: RegisterLongName> From<FieldValue<u16, R>> for u16 {
    fn from(val: FieldValue<u16, R>) -> u16 {
        val.value
    }
}

impl<R: RegisterLongName> FieldValue<u32, R> {
    pub const fn new(mask: u32, shift: usize, value: u32) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value & mask) << shift,
            associated_register: PhantomData,
        }
    }
}

impl<R: RegisterLongName> From<FieldValue<u32, R>> for u32 {
    fn from(val: FieldValue<u32, R>) -> u32 {
        val.value
    }
}

impl<R: RegisterLongName> FieldValue<u64, R> {
    pub const fn new(mask: u64, shift: usize, value: u64) -> Self {
        FieldValue {
            mask: mask << shift,
            value: (value & mask) << shift,
            associated_register: PhantomData,
        }
    }
}

impl<R: RegisterLongName> From<FieldValue<u64, R>> for u64 {
    fn from(val: FieldValue<u64, R>) -> u64 {
        val.value
    }
}

impl<T: IntLike, R: RegisterLongName> FieldValue<T, R> {
    /// Get the raw bitmask represented by this FieldValue.
    pub fn mask(self) -> T {
        self.mask as T
    }

    #[inline]
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.value)
    }

    /// Modify fields in a register value
    pub fn modify(self, val: T) -> T {
        (val & !self.mask) | self.value
    }

    /// Check if any specified parts of a field match
    pub fn matches_any(self, val: T) -> bool {
        val & self.mask != T::zero()
    }

    /// Check if all specified parts of a field match
    pub fn matches_all(self, val: T) -> bool {
        val & self.mask == self.value
    }
}

// Combine two fields with the addition operator
impl<T: IntLike, R: RegisterLongName> Add for FieldValue<T, R> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        FieldValue {
            mask: self.mask | rhs.mask,
            value: self.value | rhs.value,
            associated_register: PhantomData,
        }
    }
}

// Combine two fields with the bit or operator
impl<T: IntLike, R: RegisterLongName> BitOr for FieldValue<T, R> {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        FieldValue {
            mask: self.mask | rhs.mask,
            value: self.value | rhs.value,
            associated_register: PhantomData,
        }
    }
}

// Combine two fields with the not operator
impl<T: IntLike, R: RegisterLongName> Not for FieldValue<T, R> {
    type Output = Self;
    fn not(self) -> Self {
        FieldValue {
            mask: !self.mask,
            value: !self.value,
            associated_register: PhantomData,
        }
    }
}

// Combine two fields with the += operator
impl<T: IntLike, R: RegisterLongName> AddAssign for FieldValue<T, R> {
    fn add_assign(&mut self, rhs: FieldValue<T, R>) {
        self.mask |= rhs.mask;
        self.value |= rhs.value;
    }
}
