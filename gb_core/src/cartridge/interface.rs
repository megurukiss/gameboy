pub trait Cartridge {
    fn read_byte(&self, address: u16) -> u8;
    fn write_byte(&mut self, address: u16, value: u8);
    fn read_word(&self, address: u16) -> u16;
    fn write_word(&mut self, address: u16, value: u16);
}
