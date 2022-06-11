const RAM_SIZE: usize = 0xFFFF;

use super::gpu::*;
pub struct MemBus {
    memory: [u8; RAM_SIZE],
    gpu: GPU,
}

impl MemBus {
    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.read_vram(addr-VRAM_BEGIN)
            }
            _ => panic!("Cannot read mem")
        }
        // self.memory[addr]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(addr-VRAM_BEGIN, val)
            }
            _ => panic!("Cannot write mem")
        }
        // self.memory[addr] = val;
    }
}