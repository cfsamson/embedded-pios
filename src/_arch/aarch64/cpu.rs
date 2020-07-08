

//! Architectural processor code.

use crate::{bsp, cpu};
use cortex_a::{asm, regs::*};

// =============================================================================
// BOOT CODE
// =============================================================================

/// The entry of the `kernel` binary
/// 
/// The function must be named `_start`, because the linker is looking for this exact name
/// 
/// # Safety
/// 
/// - Linker script must ensure to place this function at `0x80_000`.

#[naked]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    use crate::runtime_init;

    // Expect the boot core to start in EL2
    if bsp::cpu::BOOT_CORE_ID == cpu::smp::core_id() {
        SP.set(bsp::cpu::BOOT_CORE_STACK_START);
        runtime_init::runtime_init()
    } else {
        // If not core0, sleep
        wait_forever();
    }
}


// -----------------------------------------------------------------------------
// PUBLIC CODE
// -----------------------------------------------------------------------------

#[inline(always)]
pub fn wait_forever() -> ! {
    loop {
        asm::wfe()
    }
}

// // SPDX-License-Identifier: MIT OR Apache-2.0
// //
// // Copyright (c) 2018-2020 Andre Richter <andre.o.richter@gmail.com>//
// .section ".text._start"//
// .global _start//
// _start:
//     mrs     x1, mpidr_el1   // Read Multiprocessor Affinity Register
//     and     x1, x1, #3      // Clear all bits except [1:0], which hold core id
//     cbz     x1, 2f          // Jump to label 2 if we are core 0
// 1:  wfe                     // Wait for event
//     b       1b              // In case an event happened, jump back to 1
// 2:                          // If we are here, we are core0
//     ldr     x1, =_start     // Load address of function "_start()"
//     mov     sp, x1          // Set start of stack to before our code, aka first
//                             // address before "_start()"
//     bl      runtime_init    // Jump to the "runtime_init()" kernel function
//     b       1b              // We should never reach here. But just in case,
//                             // park this core aswell
