use super::registers::*;
use super::mem::*;
use super::instructions::*;

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemBus,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_bype(self.pc);
        
        let prefix = instruction_byte == 0xcb;

        if prefix {
            instruction_byte = self.bus.read_bype(self.pc + 1);
        }

        let nextpc = if let Some(instruction) = Instruction::from_byte(instruction_byte, prefix) {
            self.execute(instruction)
        } else {
            panic!("Invalid instruction recieved at 0x{:x}", instruction_byte);
        };
        self.pc = nextpc;
    }

    fn execute(&mut self, instruction: Instruction) -> u16 {
        match instruction {
            Instruction::ADD(target ) => {
                match target {
                    ArithmeticTarget::C => { 
                        self.registers.a = self.add(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc
                    }
                }
            }
            Instruction::JP(target) => {
                let jumpcondition = match target {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump(jumpcondition)
            }
            Instruction::LD(loadtype) => {
                match loadtype {
                    LoadType::Byte(target,source) => {
                        let source_val = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::HLI => self.bus.read_bype(self.registers.get_hl()),
                            _ => panic!("Invalid load source recieved")
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_val,
                            LoadByteTarget::HLI => self.bus.write_byte(self.registers.get_hl(), source_val),
                            _ => panic!("Invalid load target recieved")
                        }    
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            _ => self.pc.wrapping_add(1)
                        }                     
                    }
                }
            }
            _ => {
                    self.pc
            }
        }
    }

    fn add(&mut self, n: u8) -> u8 {
        let (value, overflow) = self.registers.a.overflowing_add(n);
        self.registers.f.zero = (value == 0);
        self.registers.f.substract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xf) + (value & 0xf) > 0xf;

        value
    }

    fn jump(&mut self, jump: bool) -> u16 {
        if jump {
            let lsb = self.bus.read_bype(self.pc + 1) as u16;
            let msb = self.bus.read_bype(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
        }
    }
}
