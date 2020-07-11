
//! BSP driver support.

use crate::driver;

// -------------------------------------------------------------------------------------------------
// Public definitions
// -------------------------------------------------------------------------------------------------

/// Device Driver Manager Type
pub struct BSPDriverManager {
    device_drivers: [&'static (dyn DeviceDriver + Sync); 2],
}

// -------------------------------------------------------------------------------------------------
// Global instances
// -------------------------------------------------------------------------------------------------

static BSP_DRIVER_MANAGER: BSPDriverManager = BSPDriverManager {
    device_drivers: [&super::GPIO, &super::PL011_UART],
};

// -------------------------------------------------------------------------------------------------
// Public code
// -------------------------------------------------------------------------------------------------

/// Return a reference to the driver manager
pub fn driver_manager() -> &'static impl driver::interface::DriverManager {
    &BSP_DRIVER_MANAGER
}

// -------------------------------------------------------------------------------------------------
// OS Interface Code
// -------------------------------------------------------------------------------------------------
use driver::interface::DeviceDriver;

impl driver::interface::DriverManager for BSPDriverManager {
    fn all_device_drivers(&self) -> &[&'static (dyn DeviceDriver + Sync)] {
        &self.device_drivers[..]
    }

    fn post_device_driver_init(&self) {
        // Configure PL011Uart's output pins
        super::GPIO.map_pl011_uart();
    }
}