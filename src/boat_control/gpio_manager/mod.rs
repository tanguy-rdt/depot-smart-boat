pub mod gpio_itf;
mod gpio_manager;
mod gpio_manager_stub;

pub use gpio_manager::GpioManager;
pub use gpio_manager_stub::GpioManagerStub;

#[cfg(feature = "on_target")]
pub type Gpio = GpioManager;

#[cfg(not(feature = "on_target"))]
pub type Gpio = GpioManagerStub;
