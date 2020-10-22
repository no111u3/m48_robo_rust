use crate::mmio::{bitopts::IntLike, RegisterLongName};

use core::{cell::UnsafeCell, marker::PhantomData};

/// Read/Write registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct ReadWrite<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}

/// Read-only registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct ReadOnly<T: IntLike, R: RegisterLongName = ()> {
    value: T,
    associated_register: PhantomData<R>,
}

/// Write-only registers.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct WriteOnly<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
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

/// In memory volatile register.
// To successfully alias this structure onto hardware registers in memory, this
// struct must be exactly the size of the `T`.
#[repr(transparent)]
pub struct InMemoryRegister<T: IntLike, R: RegisterLongName = ()> {
    value: UnsafeCell<T>,
    associated_register: PhantomData<R>,
}
