


pub mod mmio;
pub mod devices_driver;

// #[cfg(feature = "board_firefly-rk3399")]
// #[path = "./firefly-rk3399/mod.rs"]
// pub mod board;

// #[cfg(feature = "board_raspi3")]
// #[path = "./raspi3/mod.rs"]
// pub mod board;

#[cfg(feature = "board_raspi4")]
#[path = "./raspi4/mod.rs"]
pub mod board;


