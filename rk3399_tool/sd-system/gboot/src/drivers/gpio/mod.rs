
pub mod grf;

use crate::bsp::map;

pub static GPIOGRF: grf::GPIOGRF = unsafe {
    grf::GPIOGRF::new(map::physical::GRF_BASE)
};

pub fn panic_gpio() -> grf::GPIOGRF {
    unsafe {
        grf::GPIOGRF::new(map::physical::GRF_BASE)
    }
}
