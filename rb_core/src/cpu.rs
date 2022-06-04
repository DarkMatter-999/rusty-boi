use super::registers::*;
use super::mem::*;
use super::instructions::*;

struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    bus: MemBus,
    is_halted: bool,
}

impl CPU {
    fn step(&mut self) {
        let mut instruction_byte = self.bus.read_byte(self.pc);
        
        let prefix = instruction_byte == 0xcb;

        if prefix {
            instruction_byte = self.bus.read_byte(self.pc + 1);
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
            Instruction::INC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self.registers.a.wrapping_add(1);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.registers.b.wrapping_add(1);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.registers.c.wrapping_add(1);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.registers.d.wrapping_add(1);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.registers.e.wrapping_add(1);
                    }
                    IncDecTarget::F => {
                        self.registers.f = FlagReg::from(u8::from(self.registers.f).wrapping_add(1));
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.registers.h.wrapping_add(1);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.registers.l.wrapping_add(1);
                    }
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let res = self.bus.read_byte(hl).wrapping_add(1);
                        self.bus.write_byte(hl, res);
                    }
                    IncDecTarget::BC => {
                        let bc = self.registers.get_bc();
                        self.registers.set_bc(bc.wrapping_add(1));
                    }
                    IncDecTarget::DE => {
                        let de = self.registers.get_de();
                        self.registers.set_de(de.wrapping_add(1));
                    }
                    IncDecTarget::HL => {
                        let hl = self.registers.get_hl();
                        self.registers.set_hl(hl.wrapping_add(1));
                    }
                    IncDecTarget::SP => {
                        let sp = self.sp;
                        self.sp = sp.wrapping_add(1);
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::DEC(target) => {
                match target {
                    IncDecTarget::A => {
                        self.registers.a = self.registers.a.wrapping_sub(1);
                    }
                    IncDecTarget::B => {
                        self.registers.b = self.registers.b.wrapping_sub(1);
                    }
                    IncDecTarget::C => {
                        self.registers.c = self.registers.c.wrapping_sub(1);
                    }
                    IncDecTarget::D => {
                        self.registers.d = self.registers.d.wrapping_sub(1);
                    }
                    IncDecTarget::E => {
                        self.registers.e = self.registers.e.wrapping_sub(1);
                    }
                    IncDecTarget::F => {
                        self.registers.f = FlagReg::from(u8::from(self.registers.f).wrapping_sub(1));
                    }
                    IncDecTarget::H => {
                        self.registers.h = self.registers.h.wrapping_sub(1);
                    }
                    IncDecTarget::L => {
                        self.registers.l = self.registers.l.wrapping_sub(1);
                    }
                    IncDecTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let res = self.bus.read_byte(hl).wrapping_sub(1);
                        self.bus.write_byte(hl, res);
                    }
                    IncDecTarget::BC => {
                        let bc = self.registers.get_bc();
                        self.registers.set_bc(bc.wrapping_sub(1));
                    }
                    IncDecTarget::DE => {
                        let de = self.registers.get_de();
                        self.registers.set_de(de.wrapping_sub(1));
                    }
                    IncDecTarget::HL => {
                        let hl = self.registers.get_hl();
                        self.registers.set_hl(hl.wrapping_sub(1));
                    }
                    IncDecTarget::SP => {
                        let sp = self.sp;
                        self.sp = sp.wrapping_sub(1);
                    }
                }
                self.pc.wrapping_add(1)
            }
            Instruction::ADD(target) => {
                match target {
                    ArithmeticTarget::B => {
                        self.registers.a = self.add(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => { 
                        self.registers.a = self.add(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.add(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.add(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::F => {
                        self.registers.a = self.add(u8::from(self.registers.f));
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.add(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.add(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc
                    }
                }
            }
            Instruction::ADDHL(target) => {
                let value = match target {
                    ADDHLTarget::BC => self.registers.get_bc(),
                    ADDHLTarget::DE => self.registers.get_de(),
                    ADDHLTarget::HL => self.registers.get_hl(),
                    ADDHLTarget::SP => self.sp,
                };
                let hl = self.registers.get_hl();
                let (result, carry) = hl.overflowing_add(value);
                
                self.registers.f.carry = carry;
                self.registers.f.substract = false;
                let mask = 0b111_1111_1111;
                self.registers.f.half_carry = (value & mask) + (hl & mask) > mask;

                self.registers.set_hl(result);
                self.pc.wrapping_add(1)
            }
            // TODO ADC
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
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
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
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    _ => panic!("Invalid stack value recieved")
                };
                self.push(value);
                self.pc.wrapping_add(1)
            }
            Instruction::CALL(target) => {
                let jumpcondition = match target {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => panic!("Invalid call value recieved")
                };
                self.call(jumpcondition)
            }
            Instruction::RET(target) => {
                let jumpcondition = match target {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                    _ => panic!("Invalid ret value recieved")
                };
                self.ret(jumpcondition)
            }
            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
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
            let lsb = self.bus.read_byte(self.pc + 1) as u16;
            let msb = self.bus.read_byte(self.pc + 2) as u16;
            (msb << 8) | lsb
        } else {
            self.pc.wrapping_add(3)
        }
    }

    fn push(&mut self, value: u16) {
        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, ((value & 0xFF00) >> 8) as u8);

        self.sp = self.sp.wrapping_sub(1);
        self.bus.write_byte(self.sp, (value & 0xFF) as u8);
    }

    fn pop(&mut self) -> u16 {
        let lsb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        let msb = self.bus.read_byte(self.sp) as u16;
        self.sp = self.sp.wrapping_add(1);

        (msb << 8) | lsb
    }

    fn call(&mut self, jump: bool) -> u16 {
        let nextpc = self.pc.wrapping_add(3);
        if jump {
            self.push(nextpc);
            self.read_next_word()
        } else {
            nextpc
        }
    }

    fn ret(&mut self, jump: bool) -> u16 {
        if jump {
            self.pop()
        } else {
            self.pc.wrapping_add(1)
        }
    }

    fn read_next_word(&self) -> u16 {
        ((self.bus.read_byte(self.pc + 2) as u16) << 8) | (self.bus.read_byte(self.pc + 1) as u16)
    }
}
