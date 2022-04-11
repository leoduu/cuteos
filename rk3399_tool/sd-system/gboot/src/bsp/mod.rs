
pub(crate) mod mmio;

#[cfg(feature = "bsp_firefly-rk3399")] 
mod rk;
#[cfg(feature = "bsp_firefly-rk3399")] 
pub use rk::*;

#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub mod raspberrypi;
#[cfg(any(feature = "bsp_rpi3", feature = "bsp_rpi4"))]
pub use raspberrypi::*;


