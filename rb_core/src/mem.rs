const RAM_SIZE: usize = 0xFFFF;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const UNUSED_BEGIN: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;

use super::gpu::*;
pub struct MemBus {
    // memory: [u8; RAM_SIZE],
    bootrom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
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
            working_ram: [0; WORKING_RAM_SIZE],
            rom_bank_0: [0; ROM_BANK_0_SIZE],
            gpu: GPU::new(),
        }
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        let addr = addr as usize;
        match addr {
            BOOT_ROM_BEGIN..=BOOT_ROM_END => {
                if let Some(boot_rom) = self.bootrom {
                    boot_rom[addr]
                } else {
                    0
                }
            }
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => self.rom_bank_0[addr],
            VRAM_BEGIN..=VRAM_END => self.gpu.vram[addr - VRAM_BEGIN],
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[addr - WORKING_RAM_BEGIN],
            ECHO_RAM_BEGIN..=ECHO_RAM_END => self.working_ram[addr - ECHO_RAM_BEGIN],
            OAM_BEGIN..=OAM_END => self.gpu.oam[addr - OAM_BEGIN],
            UNUSED_BEGIN..=UNUSED_END => {
                /* Reading this always returns 0*/
                0
            }
            _ => panic!("Cannot read mem 0x{:x}", addr)
        }
        // self.memory[addr]
    }

    pub fn write_byte(&mut self, addr: u16, val: u8) {
        let addr = addr as usize;
        match addr {
            VRAM_BEGIN ..= VRAM_END => {
                self.gpu.write_vram(addr-VRAM_BEGIN, val)
            }
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => {
                self.rom_bank_0[addr] = val;
            }
            VRAM_BEGIN..=VRAM_END => {
                self.gpu.write_vram(addr - VRAM_BEGIN, val);
            }
            WORKING_RAM_BEGIN..=WORKING_RAM_END => {
                self.working_ram[addr - WORKING_RAM_BEGIN] = val;
            }
            OAM_BEGIN..=OAM_END => {
                self.gpu.write_oam(addr - OAM_BEGIN, val);
            }
            UNUSED_BEGIN..=UNUSED_END => { /* Writing to here does nothing */ }
            
            _ => panic!("Cannot write mem 0x{:x}", addr)
        }
        // self.memory[addr] = val;
    }
}