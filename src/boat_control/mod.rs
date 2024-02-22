pub mod boat_controler_itf;
mod boat_controler;
mod boat_controler_stub;
mod bme280;
mod pca9685;
mod gpio_manager;


pub use boat_controler::BoatControler;
pub use boat_controler_stub::BoatControlerStub;
use bme280::BME280;
use pca9685::PCA9685;
use gpio_manager::Gpio;

#[cfg(feature = "on_target")]
pub type BoatControl = BoatControler;

#[cfg(not(feature = "on_target"))]
pub type BoatControl = BoatControlerStub;

