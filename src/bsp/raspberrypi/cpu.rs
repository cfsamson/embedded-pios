//! BSP Processor code.

// -----------------------------------------------------------------------------
// Public Definitions
// -----------------------------------------------------------------------------

/// Used by `arch` code to find the early boot core
pub const BOOT_CORE_ID: usize = 0;

/// The early boot core's stach address
pub const BOOT_CORE_STACK_START: u64 = 0x80_000;

/// The address on which the Raspberry firmware loads every binary by default
pub const BOARD_DEFAULT_LOAD_ADDRESS: usize = 0x80_000;