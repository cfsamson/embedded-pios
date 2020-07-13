//! PL011 UART driver.

use crate::{console, cpu, driver, synchronization::NullLock};
use core::{fmt, ops};
use register::{mmio::*, register_bitfields, register_structs};

// ------------------------- Private definitions ---------------------------------------------------

// https://github.com/raspberrypi/documentation/files/1888662/BCM2837-ARM-Peripherals.-.Revised.-.V2-1.pdf
register_bitfields! {
    u32,

    /// Flag Register
    FR [
        /// Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the
        /// Line Control Register, UARTLCR_LCRH
        ///
        /// If FIFO is disabled, this bit is set when the transmit holding register is empty. If
        /// FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does
        /// not indicate if there is data in the transmit shift register.
        TXFE OFFSET(7) NUMBITS(1) [],

        /// Transmit FIFO full. The meaning of this bit depends on the state of the FEN but in the
        /// UARTLCR_LCRH Register
        ///
        /// If FIFO is disabled, this bit is set when the transmit holding register is full. If
        /// FIFO is enabled, the TXFF bit is set when the transmit FIFO is full.
        TXFF OFFSET(5) NUMBITS(1) [],

        /// Recieve FIFO emtpy. The meaning of this bit depends on the state of the FEN bit in the
        /// UARTLCR_H Register.
        ///
        /// If FIFO is disabled, this bit is set when the recieve holding register is empty. If
        /// FIFO is enabled, the RXFE bit is set when the recieve FIFO is empty.
        RXFE OFFSET(4) NUMBITS(1) []
    ],

    /// Integere Baud rate divisor
    IBRD [
        /// Integer Baud rate divisor
        IBRD OFFSET(0) NUMBITS(16) []
    ],

    /// Fractional Baud rate divisor
    FBRD [
        /// Fractional Baud rate divisor
        FBRD OFFSET(0) NUMBITS(6) []
    ],

    /// Line Control register
    LCRH [
        /// Word length. These bits indicate the number of data bits transmitted or recieved in a
        /// frame.
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBit = 0b00,
            SixBit = 0b01,
            SevenBit = 0b10,
            EightBit = 0b11
        ],

        /// Enable FIFOs:
        ///
        /// 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding
        /// registers.
        ///
        /// 1 = transmit and recieve FIFO buffers are enabled (FIFO mode).
        FEN OFFSET(4) NUMBITS(1) [
            FifosDisabled = 0,
            FifosEnabled = 1
        ]
    ],

    /// Control Register
    CR [
        /// Recieve enable. If this bit is set to 1, the recieve section of the UART is enabled.
        /// Data reception occurs for UART signals. When the UART is disabled in the middle of
        /// reception, it complets the current character before stopping.
        RXE OFFSET(9) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled.
        /// Data transmission occurs for UART signals. When the UART is disabled in the middle of
        /// transmission, it completes the current character before stopping.
        TXE OFFSET(8) NUMBITS(1) [
            Disabled = 0,
            Enabled = 1
        ],

        /// UART enable
        UARTEN OFFSET(0) NUMBITS(1) [
            /// If the UART is disabled in the middle of transmission or reception. it completes the
            /// current character before stopping.
            Disabled = 0,
            Enabled = 1
        ]
    ],

    /// Interrupt Clear Register
    ICR [
        /// Meta field for all pending interrupts
        ALL OFFSET(0) NUMBITS(11) []
    ]
}

// 0000000001111111112222222223333333334444444445555555556666
// 1234567891234567891234567891234567891234567891234567891234
// ----------------------------------------------------------

// ---------------------------- Public definitions -------------------------------------------------

register_structs! {
    #[allow(non_snake_case)]
    pub RegisterBlock {
        (0x00 => DR: ReadWrite<u32>),
        (0x04 => _reserved),
        (0x18 => FR: ReadOnly<u32, FR::Register>),
        (0x1c => _reserved2),
        (0x24 => IBRD: WriteOnly<u32, IBRD::Register>),
        (0x28 => FBRD: WriteOnly<u32, FBRD::Register>),
        (0x2c => LCRH: WriteOnly<u32, LCRH::Register>),
        (0x30 => CR: WriteOnly<u32, CR::Register>),
        (0x34 => _reserved3),
        (0x44 => ICR: WriteOnly<u32, ICR::Register>),
        (0x48 => @END),
    }
}

pub struct PL011UartInner {
    base_addr: usize,
    chars_written: usize,
    chars_read: usize,
}

// Export the inner struct so that BSPs can use it for the panic handler
pub use PL011UartInner as PanicUart;

// Representation of the UART
pub struct PL011Uart {
    inner: NullLock<PL011UartInner>,
}

// ----------------------------------- PUBLIC CODE -------------------------------------------------

/// Deref to RegisterBlock.
///
/// Allows writing
/// ```
/// self.DR.read()
/// ```
/// instead of something along the lines of
/// ```
/// unsafe { (*PL011UartInner::ptr()).DR.read() }
impl ops::Deref for PL011UartInner {
    type Target = RegisterBlock;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr() }
    }
}

impl PL011UartInner {
    /// Create an instance.
    ///
    /// # Safety
    ///
    /// - The user must ensure to provide the correct `base_addr`.
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            base_addr,
            chars_written: 0,
            chars_read: 0,
        }
    }

    /// Set up baud rate and characteristics.
    ///
    ///
    /// Results in 8N1 and 230400 baud (if the clk has been previously set to 48 MHz by the
    /// firmware).
    pub fn init(&mut self) {
        // Turn if off temporarily
        self.CR.set(0);

        self.ICR.write(ICR::ALL::CLEAR);
        self.IBRD.write(IBRD::IBRD.val(13));
        self.FBRD.write(FBRD::FBRD.val(2));
        self.LCRH
            .write(LCRH::WLEN::EightBit + LCRH::FEN::FifosEnabled); // 8NI + FIFO on
        self.CR
            .write(CR::UARTEN::Enabled + CR::TXE::Enabled + CR::RXE::Enabled);
    }

    /// Return a pointer to the register block
    fn ptr(&self) -> *const RegisterBlock {
        self.base_addr as *const _
    }

    /// Send a character
    fn write_char(&mut self, c: char) {
        // Spin while TX FIFO full is set, waiting for an empty slot
        while self.FR.matches_all(FR::TXFF::SET) {
            cpu::nop();
        }

        // Write the character to the buffer
        self.DR.set(c as u32);
        self.chars_written += 1;
    }
}

/// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
/// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
/// we get `write_fmt()` automatically.
///
/// The function takes an `&mut self`, so it must be implemented for the inner struct.
///
/// See [`src/print.rs`].
///
/// [`src/print.rs`]: ../../print/index.html
impl fmt::Write for PL011UartInner {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            self.write_char(c);
        }

        Ok(())
    }
}

impl PL011Uart {
    /// # Safety
    ///
    /// - The user must ensure to provide the correct `base_addr`.
    pub const unsafe fn new(base_addr: usize) -> Self {
        Self {
            inner: NullLock::new(PL011UartInner::new(base_addr)),
        }
    }
}

// ------------------------------ OS INTERFACE CODE ------------------------------------------------
use crate::synchronization::interface::Mutex;

impl driver::interface::DeviceDriver for PL011Uart {
    fn compatible(&self) -> &str {
        "BCM PL011 UART"
    }

    fn init(&self) -> Result<(), ()> {
        let mut r = &self.inner;
        r.lock(|inner| inner.init());

        Ok(())
    }
}

impl console::interface::Write for PL011Uart {
    /// Passtrhough of `args` to the `core::fmt::Write` impl, but guarded by a Mutex to
    /// serialize access.
    fn write_char(&self, c: char) {
        let mut r = &self.inner;

        r.lock(|inner| inner.write_char(c));
    }

    fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
        // Fully qualified syntax for the call to `core::fmt::Write::write_fmt()` to increase
        // readability
        let mut r = &self.inner;
        r.lock(|inner| fmt::Write::write_fmt(inner, args))
    }

    fn flush(&self) {
        // Spin until TX FIFO empty is set
        let mut r = &self.inner;
        r.lock(|inner| {
            while !inner.FR.matches_all(FR::TXFE::SET) {
                cpu::nop();
            }
        })
    }
}

impl console::interface::Read for PL011Uart {
    fn read_char(&self) -> char {
        let mut r = &self.inner;

        r.lock(|inner| {
            // Spin while RX FIFO empty is set
            while inner.FR.matches_all(FR::RXFE::SET) {
                cpu::nop();
            }

            // Read one character
            inner.DR.get() as u8 as char
        })
    }

    fn clear(&self) {
        let mut r = &self.inner;
        r.lock(|inner| {
            // Read from the RX FIFO until it is indicating empty.
            while !inner.FR.matches_all(FR::RXFE::SET) {
                inner.DR.get();
            }
        })
    }
}

impl console::interface::Statistics for PL011Uart {
    fn chars_written(&self) -> usize {
        let mut r = &self.inner;
        r.lock(|inner| inner.chars_written)
    }

    fn chars_read(&self) -> usize {
        let mut r = &self.inner;
        r.lock(|inner| inner.chars_read)
    }
}
