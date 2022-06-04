const RAM_SIZE: usize = 0xFFFF;

pub struct MemBus {
    memory: [u8; RAM_SIZE],
}

impl MemBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}