//! Symmetric multiprocessing.

#[cfg(target_arch = "aarch64")]
#[path = "../_arch/aarch64/cpu/smp.rs"]
mod arch_cpu_smp;
pub use arch_cpu_smp::*;