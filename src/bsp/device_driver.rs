#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
mod bcm;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use bcm::*;