use crate::mmio::{
    bitopts::{Field, FieldValue, IntLike},
    RegisterLongName, TryFromValue,
};

use core::{cell::UnsafeCell, fmt, marker::PhantomData};

/// Read/Write registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct ReadWrite<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}

impl<T: IntLike, R: RegisterLongName> ReadWrite<T, R> {
    #[inline]
    /// Get the raw register value
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    /// Set the raw register value
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(self.value.get(), value) }
    }

    #[inline]
    /// Read the value of the given field
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    /// Read value of the given field as an enum member
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    /// Make a local copy of the register
    pub fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    /// Write the value of one or more fields, overwriting the other fields with zero
    pub fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }

    #[inline]
    /// Write the value of one or more fields, leaving the other fields unchanged
    pub fn modify(&self, field: FieldValue<T, R>) {
        self.set(field.modify(self.get()));
    }

    #[inline]
    /// Write the value of one or more fields, maintaining the value of unchanged fields via a
    /// provided original value, rather than a register read.
    pub fn modify_no_read(&self, original: LocalRegisterCopy<T, R>, field: FieldValue<T, R>) {
        self.set(field.modify(original.get()));
    }

    #[inline]
    /// Check if one or more bits in a field are set
    pub fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    /// Check if any specified parts of a field match
    pub fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    /// Check if all specified parts of a field match
    pub fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}

/// Read-only registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct ReadOnly<T: IntLike, R: RegisterLongName = ()> {
    value: T,
    associated_register: PhantomData<R>,
}

impl<T: IntLike, R: RegisterLongName> ReadOnly<T, R> {
    #[inline]
    /// Get the raw register value
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(&self.value) }
    }

    #[inline]
    /// Read the value of the given field
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    /// Read value of the given field as an enum member
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    /// Make a local copy of the register
    pub fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    /// Check if one or more bits in a field are set
    pub fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    /// Check if any specified parts of a field match
    pub fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    /// Check if all specified parts of a field match
    pub fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}

/// Write-only registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct WriteOnly<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}

impl<T: IntLike, R: RegisterLongName> WriteOnly<T, R> {
    #[inline]
    /// Set the raw register value
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(self.value.get(), value) }
    }

    #[inline]
    /// Write the value of one or more fields, overwriting the other fields with zero
    pub fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }
}

/// Read-only and write-only registers aliased to the same address.
///
/// Unlike the `ReadWrite` register, this represents a register which has different meanings based
/// on if it is written or read.  This might be found on a device where control and status
/// registers are accessed via the same memory address via writes and reads, respectively.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct Aliased<T: IntLike, R: RegisterLongName = (), W: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<(R, W)>,
}

impl<T: IntLike, R: RegisterLongName, W: RegisterLongName> Aliased<T, R, W> {
    #[inline]
    /// Get the raw register value
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    /// Set the raw register value
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(self.value.get(), value) }
    }

    #[inline]
    /// Read the value of the given field
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    /// Read value of the given field as an enum member
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    /// Make a local copy of the register
    pub fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    /// Write the value of one or more fields, overwriting the other fields with zero
    pub fn write(&self, field: FieldValue<T, W>) {
        self.set(field.value);
    }

    #[inline]
    /// Check if one or more bits in a field are set
    pub fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    /// Check if any specified parts of a field match
    pub fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    /// Check if all specified parts of a field match
    pub fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}

/// A read-only copy register contents
///
/// This behaves very similarly to a read-only register, but instead of doing a
/// volatile read to MMIO to get the value for each function call, a copy of the
/// register contents are stored locally in memory. This allows a peripheral
/// to do a single read on a register, and then check which bits are set without
/// having to do a full MMIO read each time. It also allows the value of the
/// register to be "cached" in case the peripheral driver needs to clear the
/// register in hardware yet still be able to check the bits.
#[derive(Copy, Clone)]
pub struct LocalRegisterCopy<T: IntLike, R: RegisterLongName = ()> {
    value: T,
    associated_register: PhantomData<R>,
}

impl<T: IntLike, R: RegisterLongName> LocalRegisterCopy<T, R> {
    pub const fn new(value: T) -> Self {
        LocalRegisterCopy {
            value: value,
            associated_register: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self) -> T {
        self.value
    }

    #[inline]
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    pub fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    pub fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    pub fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }

    /// Do a bitwise AND operation of the stored value and the passed in value
    /// and return a new LocalRegisterCopy.
    #[inline]
    pub fn bitand(&self, rhs: T) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.value & rhs)
    }
}

impl<T: IntLike + fmt::Debug, R: RegisterLongName> fmt::Debug for LocalRegisterCopy<T, R> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.value)
    }
}

impl<R: RegisterLongName> From<LocalRegisterCopy<u8, R>> for u8 {
    fn from(r: LocalRegisterCopy<u8, R>) -> u8 {
        r.value
    }
}

impl<R: RegisterLongName> From<LocalRegisterCopy<u16, R>> for u16 {
    fn from(r: LocalRegisterCopy<u16, R>) -> u16 {
        r.value
    }
}

impl<R: RegisterLongName> From<LocalRegisterCopy<u32, R>> for u32 {
    fn from(r: LocalRegisterCopy<u32, R>) -> u32 {
        r.value
    }
}

impl<R: RegisterLongName> From<LocalRegisterCopy<u64, R>> for u64 {
    fn from(r: LocalRegisterCopy<u64, R>) -> u64 {
        r.value
    }
}

/// In memory volatile register.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct InMemoryRegister<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}

impl<T: IntLike, R: RegisterLongName> InMemoryRegister<T, R> {
    pub const fn new(value: T) -> Self {
        InMemoryRegister {
            value: UnsafeCell::new(value),
            associated_register: PhantomData,
        }
    }

    #[inline]
    pub fn get(&self) -> T {
        unsafe { ::core::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    pub fn set(&self, value: T) {
        unsafe { ::core::ptr::write_volatile(self.value.get(), value) }
    }

    #[inline]
    pub fn read(&self, field: Field<T, R>) -> T {
        field.read(self.get())
    }

    #[inline]
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(&self, field: Field<T, R>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    pub fn extract(&self) -> LocalRegisterCopy<T, R> {
        LocalRegisterCopy::new(self.get())
    }

    #[inline]
    pub fn write(&self, field: FieldValue<T, R>) {
        self.set(field.value);
    }

    #[inline]
    pub fn modify(&self, field: FieldValue<T, R>) {
        self.set(field.modify(self.get()));
    }

    #[inline]
    pub fn modify_no_read(&self, original: LocalRegisterCopy<T, R>, field: FieldValue<T, R>) {
        self.set(field.modify(original.get()));
    }

    #[inline]
    pub fn is_set(&self, field: Field<T, R>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    pub fn matches_any(&self, field: FieldValue<T, R>) -> bool {
        field.matches_any(self.get())
    }

    #[inline]
    pub fn matches_all(&self, field: FieldValue<T, R>) -> bool {
        field.matches_all(self.get())
    }
}
