
//! GPIO Driver.

use crate::{cpu, driver, synchronization::NullLock};
use core::ops;
use register::{mmio::*, register_bitfields, register_structs};

// -------------------------------------------------------------------------------------------------
// Private Definitions
// -------------------------------------------------------------------------------------------------

register_bitfields! {
    u32,

    /// GPIO Function Select 1
    GPFSEL1 [
        /// Pin 15
        FSEL15 OFFSET(15) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART RX
        ],

        /// Pin 14
        FSEL14 OFFSET(12) NUMBITS(3) [
            Input = 0b000,
            Output = 0b001,
            AltFunc0 = 0b100 // PL011 UART TX
        ]
    ],

    /// GPIO Pull-up/down Clock Register 0
    GPPUDCLK0 [
        /// Pin 15
        PUDCLK15 OFFSET(15) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ],

        // Pin 14
        PUDCLK14 OFFSET(14) NUMBITS(1) [
            NoEffect = 0,
            AssertClock = 1
        ]
    ]
}

register_structs! {
    #[allow(non_snake_case)]
    RegisterBlock {
        (0x00 => GPFSEL0: ReadWrite<u32>),
        (0x04 => GPFSEL1: ReadWrite<u32, GPFSEL1::Register>),
        (0x08 => GPFSEL2: ReadWrite<u32>),
        (0x0C => GPFSEL3: ReadWrite<u32>),
        (0x10 => GPFSEL4: ReadWrite<u32>),
        (0x14 => GPFSEL5: ReadWrite<u32>),
        (0x18 => _reserved1),
        (0x94 => GPPUD: ReadWrite<u32>),
        (0x98 => GPPUDCLK0: ReadWrite<u32, GPPUDCLK0::Register>),
        (0x9C => GPPUDCLK1: ReadWrite<u32>),
        (0xA0 => @END),
    }
}

struct GPIOInner {
    base_addr: usize,
}

// -------------------------------------------------------------------------------------------------
// Public Definitions
// -------------------------------------------------------------------------------------------------

/// Representation of the GPIO HW
pub struct GPIO {
    inner: NullLock<GPIOInner>,
}

// -------------------------------------------------------------------------------------------------
// Private code
// -------------------------------------------------------------------------------------------------

impl ops::Deref for GPIOInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl GPIOInner {
    const fn new(base_addr: usize) -> Self {
        Self { base_addr }
    }

    /// Return a pointer to the associated MMIO register block.
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }
}


// -------------------------------------------------------------------------------------------------
// Public code
// -------------------------------------------------------------------------------------------------

impl GPIO {

    /// Crate an instance
    /// 
    /// # Safety
    /// 
    /// - The user must ensure to provide the correct `base_adr`
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            inner: NullLock::new(GPIOInner::new(base_addr))
        }
    }

    pub fn map_pl011_uart(&self) {
        let mut r = &self.inner;
        r.lock(|inner| {
            // Map to pins
            inner
                .GPFSEL1
                .modify(GPFSEL1::FSEL14::AltFunc0 + GPFSEL1::FSEL15::AltFunc0);

                // Enable pins 14 and 15
                inner.GPPUD.set(0);
                cpu::spin_for_cycles(150);

                inner.GPPUDCLK0.write(GPPUDCLK0::PUDCLK14::AssertClock + GPPUDCLK0::PUDCLK15::AssertClock);
                cpu::spin_for_cycles(150);

                inner.GPPUDCLK0.set(0);
        })
    }
}


// -------------------------------------------------------------------------------------------------
// OS Interface Code
// -------------------------------------------------------------------------------------------------

use crate::synchronization::interface::Mutex;

impl driver::interface::DeviceDriver for GPIO {
    fn compatible(&self) -> &str {
        "BCM GPIO"
    }
}