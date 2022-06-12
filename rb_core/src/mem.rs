const RAM_SIZE: usize = 0xFFFF;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

use super::gpu::*;
pub struct MemBus {
    // memory: [u8; RAM_SIZE],
    bootrom: Option<[u8; BOOT_ROM_SIZE]>,
    gpu: GPU,
}

impl MemBus {
    pub fn new(bootrombuffer: Option<Vec<u8>>, gamerombuffer: Option<Vec<u8>>) -> MemBus {
        let boot_rom = bootrombuffer.map(|bootrombuffer| {
            if bootrombuffer.len() != BOOT_ROM_SIZE {
                panic!("BootROM size wrong\n expected {} bytes got {} bytes", BOOT_ROM_SIZE, bootrombuffer.len());
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&bootrombuffer);
            boot_rom
        });

        MemBus {
            bootrom: boot_rom,
            gpu: GPU::new(),
        }
    }
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