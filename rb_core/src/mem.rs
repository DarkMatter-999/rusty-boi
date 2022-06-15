const RAM_SIZE: usize = 0xFFFF;

pub const BOOT_ROM_BEGIN: usize = 0x00;
pub const BOOT_ROM_END: usize = 0xFF;
pub const BOOT_ROM_SIZE: usize = BOOT_ROM_END - BOOT_ROM_BEGIN + 1;

pub const ROM_BANK_0_BEGIN: usize = 0x0000;
pub const ROM_BANK_0_END: usize = 0x3FFF;
pub const ROM_BANK_0_SIZE: usize = ROM_BANK_0_END - ROM_BANK_0_BEGIN + 1;

pub const ROM_BANK_N_BEGIN: usize = 0x4000;
pub const ROM_BANK_N_END: usize = 0x7FFF;
pub const ROM_BANK_N_SIZE: usize = ROM_BANK_N_END - ROM_BANK_N_BEGIN + 1;

pub const WORKING_RAM_BEGIN: usize = 0xC000;
pub const WORKING_RAM_END: usize = 0xDFFF;
pub const WORKING_RAM_SIZE: usize = WORKING_RAM_END - WORKING_RAM_BEGIN + 1;

pub const ECHO_RAM_BEGIN: usize = 0xE000;
pub const ECHO_RAM_END: usize = 0xFDFF;

pub const UNUSED_BEGIN: usize = 0xFEA0;
pub const UNUSED_END: usize = 0xFEFF;

pub const IO_REGISTERS_BEGIN: usize = 0xFF00;
pub const IO_REGISTERS_END: usize = 0xFF7F;

pub const ZERO_PAGE_BEGIN: usize = 0xFF80;
pub const ZERO_PAGE_END: usize = 0xFFFE;
pub const ZERO_PAGE_SIZE: usize = ZERO_PAGE_END - ZERO_PAGE_BEGIN + 1;


pub const EXTERNAL_RAM_BEGIN: usize = 0xA000;
pub const EXTERNAL_RAM_END: usize = 0xBFFF;
pub const EXTERNAL_RAM_SIZE: usize = EXTERNAL_RAM_END - EXTERNAL_RAM_BEGIN + 1;

pub const INTERRUPT_ENABLE_REGISTER: usize = 0xFFFF;

use super::gpu::*;
pub struct MemBus {
    // memory: [u8; RAM_SIZE],
    pub bootrom: Option<[u8; BOOT_ROM_SIZE]>,
    rom_bank_0: [u8; ROM_BANK_0_SIZE],
    rom_bank_n: [u8; ROM_BANK_N_SIZE],
    working_ram: [u8; WORKING_RAM_SIZE],
    zero_page: [u8; ZERO_PAGE_SIZE],
    pub gpu: GPU,
}

impl MemBus {
    pub fn new(bootrombuffer: Option<Vec<u8>>, gamerombuffer: Vec<u8>) -> MemBus {
        let boot_rom = bootrombuffer.map(|bootrombuffer| {
            if bootrombuffer.len() != BOOT_ROM_SIZE {
                panic!("BootROM size wrong\n expected {} bytes got {} bytes", BOOT_ROM_SIZE, bootrombuffer.len());
            }
            let mut boot_rom = [0; BOOT_ROM_SIZE];
            boot_rom.copy_from_slice(&bootrombuffer);
            boot_rom
        });

        let mut rom_bank0 = [0; ROM_BANK_0_SIZE];
        for i in 0..ROM_BANK_0_SIZE {
            rom_bank0[i] = gamerombuffer[i];
        }
        let mut rom_bankn = [0; ROM_BANK_N_SIZE];
        for i in 0..ROM_BANK_N_SIZE {
            rom_bankn[i] = gamerombuffer[ROM_BANK_0_SIZE + i];
        }
        MemBus {
            bootrom: boot_rom,
            working_ram: [0; WORKING_RAM_SIZE],
            rom_bank_0: rom_bank0,
            rom_bank_n: rom_bankn,
            zero_page: [0; ZERO_PAGE_SIZE],
            gpu: GPU::new(),
        }
    }
    pub fn step(&mut self, cycles: u8) {
        self.gpu.step(cycles);
            
    }
    pub fn read_byte(&self, addr: u16) -> u8 {
        // println!("0x{:x}", addr);
        // if addr == 260 {
        //     println!("{:x}", self.rom_bank_0[(addr+1) as usize]);
        //     panic!("reading dmg")
        // }
        let addr = addr as usize;
        match addr {
            BOOT_ROM_BEGIN..=BOOT_ROM_END => {
                // return self.rom_bank_0[addr];
                if let Some(boot_rom) = self.bootrom {
                    boot_rom[addr]
                } else {
                    self.rom_bank_0[addr]
                }
            }
            ROM_BANK_0_BEGIN..=ROM_BANK_0_END => self.rom_bank_0[addr],
            ROM_BANK_N_BEGIN..=ROM_BANK_N_END => self.rom_bank_n[addr - ROM_BANK_N_BEGIN],
            VRAM_BEGIN..=VRAM_END => self.gpu.vram[addr - VRAM_BEGIN],
            WORKING_RAM_BEGIN..=WORKING_RAM_END => self.working_ram[addr - WORKING_RAM_BEGIN],
            ECHO_RAM_BEGIN..=ECHO_RAM_END => self.working_ram[addr - ECHO_RAM_BEGIN],
            OAM_BEGIN..=OAM_END => self.gpu.oam[addr - OAM_BEGIN],
            UNUSED_BEGIN..=UNUSED_END => {0}
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => self.read_io_register(addr),
            ZERO_PAGE_BEGIN..=ZERO_PAGE_END => self.zero_page[addr - ZERO_PAGE_BEGIN],
            INTERRUPT_ENABLE_REGISTER => 0,
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
            IO_REGISTERS_BEGIN..=IO_REGISTERS_END => {
                self.write_io_register(addr, val);
            }
            ZERO_PAGE_BEGIN..=ZERO_PAGE_END => {
                self.zero_page[addr - ZERO_PAGE_BEGIN] = val;
            }
            INTERRUPT_ENABLE_REGISTER => (),
            _ => panic!("Cannot write mem 0x{:x}", addr)
        }
        // self.memory[addr] = val;
    }
    fn read_io_register(&self, address: usize) -> u8 { self.gpu.line }
    fn write_io_register(&mut self, address: usize, value: u8) {}

}