
pub trait Interface {

    fn init(&self);
    
    fn read(&self) -> u8;

    fn write(&self, data: u8);
    
    fn write_drain(&self);

    fn flush(&self);
}



