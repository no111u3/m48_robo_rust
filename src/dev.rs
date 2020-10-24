macro_rules! bitmask {
    ($numbits:expr) => {
        (1 << ($numbits - 1)) + ((1 << ($numbits - 1)) - 1)
    };
}

pub mod gpio {
    use crate::mmio::{
        registers::ReadOnly, registers::ReadWrite, static_ref::StaticRef, RegisterLongName,
    };

    #[repr(C)]
    pub struct GpioBRegisters {
        pub pinb: ReadOnly<u8, PINB>,
        pub ddrb: ReadWrite<u8, DDRB::Register>,
        pub portb: ReadWrite<u8, PORTB::Register>,
    }

    pub struct PINB {}

    impl RegisterLongName for PINB {}

    pub mod DDRB {
        pub struct Register {}

        impl super::RegisterLongName for Register {}

        pub mod DDB0 {
            use crate::mmio::bitopts::FieldValue;

            pub const SET: FieldValue<u8, super::Register> =
                FieldValue::<u8, super::Register>::new(bitmask!(1), 0, bitmask!(1));

            pub const CLEAR: FieldValue<u8, super::Register> =
                FieldValue::<u8, super::Register>::new(bitmask!(1), 0, 0);
        }
    }

    pub mod PORTB {
        pub struct Register {}

        impl super::RegisterLongName for Register {}

        pub mod PORTB0 {
            use crate::mmio::bitopts::FieldValue;

            pub const SET: FieldValue<u8, super::Register> =
                FieldValue::<u8, super::Register>::new(bitmask!(1), 0, bitmask!(1));

            pub const CLEAR: FieldValue<u8, super::Register> =
                FieldValue::<u8, super::Register>::new(bitmask!(1), 0, 0);
        }
    }

    pub const GPIOB: StaticRef<GpioBRegisters> =
        unsafe { StaticRef::new(0x23 as *const GpioBRegisters) };
}
