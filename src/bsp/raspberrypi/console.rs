//! BSP console facilities.

use super::memory;
use crate::{bsp::device_driver, console};
use core::fmt;


// -------------------------------------------------------------------------------------------------
// Public code
// -------------------------------------------------------------------------------------------------

/// In case of a panic, the panic handler uses this function to take a last shot at priting
/// something befire the system is halted
/// 
/// # Safety
/// 
/// - Use only for priting during a panic
pub unsafe fn panic_console_out() -> impl fmt::Write {
    let mut uart = device_driver::PanicUart::new(memory::map::mmio::PL011_UART_BASE);
    uart.init();
    uart
}

pub fn console() -> &'static impl console::interface::All {
    &super::PL011_UART
}

// use crate::{console, synchronization, synchronization::NullLock};
// use core::fmt;

// // =============================================================================
// // PRIVATE DEFINITIONS
// // =============================================================================

// /// A mystical, magical device for generating QEMU output out of the void.

// struct QEMUOutputInner {
//     chars_written: usize,
// }

// // =============================================================================
// // Public definitions
// // =============================================================================

// /// The main struct
// pub struct QEMUOutput {
//     inner: NullLock<QEMUOutputInner>,
// }

// // =============================================================================
// // Glocal Instances
// // =============================================================================

// static QEMU_OUTPUT: QEMUOutput = QEMUOutput::new();

// // =============================================================================
// // PRIVATE IMPLEMENTATIONS
// // =============================================================================

// impl QEMUOutputInner {
//     const fn new() -> QEMUOutputInner {
//         QEMUOutputInner { chars_written: 0 }
//     }

//     fn write_char(&mut self, c: char) {
//         unsafe {
//             core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
//         }

//         self.chars_written += 1;
//     }
// }

// /// Implementing `core::fmt::Write` enables usage of the `format_args!` macros, which in turn are
// /// used to implement the `kernel`'s `print!` and `println!` macros. By implementing `write_str()`,
// /// we get `write_fmt()` automatically.
// /// 
// ///
// /// The function takes an `&mut self`, so it must be implemented for the inner struct.
// ///
// /// See [`src/print.rs`].
// ///
// /// [`src/print.rs`]: ../../print/index.html
// impl fmt::Write for QEMUOutputInner {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         for c in s.chars() {
//             // Convert newline to carrige return + newline.
//             if c == '\n' {
//                 self.write_char('\r');
//             }

//             self.write_char(c);
//         }

//         Ok(())
//     }
// }

// // =============================================================================
// // PUBLIC CODE
// // =============================================================================

// impl QEMUOutput {
//     /// Crate a new instance
//     pub const fn new() -> QEMUOutput {
//         QEMUOutput {
//             inner: NullLock::new(QEMUOutputInner::new()),
//         }
//     }
// }

// pub fn console() -> &'static impl console::interface::All {
//     &QEMU_OUTPUT
// }

// // =============================================================================
// // OS INTERFACE CODE
// // =============================================================================
// use synchronization::interface::Mutex;

// /// Passthrough of `args` to the `core::fmt::Write` implementation, but guarded by a Mutex to
// /// serialize access.
// impl console::interface::Write for QEMUOutput {
//     fn write_fmt(&self, args: core::fmt::Arguments) -> fmt::Result {
//         // Fully qualified syntax for the call to `core::fmt::Write::write_fmt()` to increase
//         // readability
//         let mut r = &self.inner;
//         r.lock(|inner| fmt::Write::write_fmt(inner, args))
//     }
// }

// impl console::interface::Statistics for QEMUOutput {
//     fn chars_written(&self) -> usize {
//         let mut r = &self.inner;
//         r.lock(|inner| inner.chars_written)
//     }
// }
