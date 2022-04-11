
pub trait IRQHandler {
    /// Called when the corresponding interrupt is asserted.
    fn handle(&self) -> Result<(), &'static str>;
}

pub trait IRQManager {
    
    fn register_irq(&self, irq_num: usize, handler: fn()) -> Result<(), &'static str>;

    fn enable_irq(&self, irq_num: usize);

}
