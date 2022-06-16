use super::registers::*;
use super::mem::*;
use super::instructions::*;

use super::gpu::{SCREEN_HEIGHT, SCREEN_WIDTH};
pub struct CPU {
    registers: Registers,
    pc: u16,
    sp: u16,
    pub bus: MemBus,
    is_halted: bool,
    interrupts_enabled: bool,
}

impl CPU {
    pub fn new(bootrombuffer: Option<Vec<u8>>, gamerombuffer: Vec<u8>) -> CPU {
        CPU { registers: Registers::new(),
            pc: 0x0,
            sp: 0x00,
            bus: MemBus::new(bootrombuffer, gamerombuffer),
            is_halted: false,
            interrupts_enabled: true,
        }
    }
    pub fn step(&mut self) {
        
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
        self.bus.step(1);

        // println!("{} 0x{:x}", prefix, instruction_byte);

        if self.bus.has_interrupt() {
            self.is_halted = false;
        }

        if !self.is_halted {
            self.pc = nextpc;
        }

        let mut interrupted = false;
        if self.interrupts_enabled {
            if self.bus.interrupt_enable.vblank && self.bus.interrupt_flag.vblank {
                interrupted = true;
                self.bus.interrupt_flag.vblank = false;
                self.interrupt(VBLANK_VECTOR)
            }
            if self.bus.interrupt_enable.lcdstat && self.bus.interrupt_flag.lcdstat {
                interrupted = true;
                self.bus.interrupt_flag.lcdstat = false;
                self.interrupt(LCDSTAT_VECTOR)
            }
            if self.bus.interrupt_enable.timer && self.bus.interrupt_flag.timer {
                interrupted = true;
                self.bus.interrupt_flag.timer = false;
                self.interrupt(TIMER_VECTOR)
            }
        }
    }

    fn interrupt(&mut self, location: u16) {
        self.interrupts_enabled = false;
        self.push(self.pc);
        self.pc = location;
        self.bus.step(12);
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
                    ArithmeticTarget::A => {
                        self.registers.a = self.add(self.registers.a);
                        self.pc.wrapping_add(1)
                    }
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
                    ArithmeticTarget::D8 => {
                        self.registers.a = self.add(self.read_next_byte());
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.a = self.add(self.bus.read_byte(self.registers.get_hl()));
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
            Instruction::ADC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.add_with_carry(self.registers.a);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.add_with_carry(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => { 
                        self.registers.a = self.add_with_carry(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.add_with_carry(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.add_with_carry(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::F => {
                        self.registers.a = self.add_with_carry(u8::from(self.registers.f));
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.add_with_carry(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.add_with_carry(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D8 => {
                        self.registers.a = self.add_with_carry(self.read_next_byte());
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.a = self.add_with_carry(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc
                    }
                }
            }
            Instruction::ADDSP => {
                let value = self.read_next_byte() as i8 as i16 as u16;
                let res = self.sp.wrapping_add(value);
                
                self.registers.f.half_carry = (self.sp & 0xf) + (value & 0xf) > 0xf;

                self.registers.f.carry = (self.sp & 0xff) + (value & 0xff) > 0xff;
                self.registers.f.zero = false;
                self.registers.f.substract = false;

                self.sp = res;

                self.pc.wrapping_add(2)
            }
            Instruction::SUB(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.sub(self.registers.a);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.sub(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => { 
                        self.registers.a = self.sub(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.sub(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.sub(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::F => {
                        self.registers.a = self.sub(u8::from(self.registers.f));
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.sub(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.sub(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D8 => {
                        self.registers.a = self.sub(self.read_next_byte());
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.a = self.sub(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc
                    }
                }
            }
            Instruction::SBC(target) => {
                match target {
                    ArithmeticTarget::A => {
                        self.registers.a = self.sub_with_carry(self.registers.a);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::B => {
                        self.registers.a = self.sub_with_carry(self.registers.b);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::C => { 
                        self.registers.a = self.sub_with_carry(self.registers.c);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D => {
                        self.registers.a = self.sub_with_carry(self.registers.d);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::E => {
                        self.registers.a = self.sub_with_carry(self.registers.e);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::F => {
                        self.registers.a = self.sub_with_carry(u8::from(self.registers.f));
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::H => {
                        self.registers.a = self.sub_with_carry(self.registers.h);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::L => {
                        self.registers.a = self.sub_with_carry(self.registers.l);
                        self.pc.wrapping_add(1)
                    }
                    ArithmeticTarget::D8 => {
                        self.registers.a = self.sub_with_carry(self.read_next_byte());
                        self.pc.wrapping_add(2)
                    }
                    ArithmeticTarget::HLI => {
                        self.registers.a = self.sub_with_carry(self.bus.read_byte(self.registers.get_hl()));
                        self.pc.wrapping_add(1)
                    }
                    _ => {
                        self.pc
                    }
                }
            }
            Instruction::AND(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::F => u8::from(self.registers.f),
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::D8 => self.read_next_byte(),
                    ArithmeticTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
                };
                let n = self.registers.a & value;
                self.registers.f.zero = n == 0;
                self.registers.f.substract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
                self.registers.a = n;
                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
                
            }
            Instruction::OR(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::F => u8::from(self.registers.f),
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::D8 => self.read_next_byte(),
                    ArithmeticTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
                };
                let n = self.registers.a | value;
                self.registers.f.zero = n == 0;
                self.registers.f.substract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
                self.registers.a = n;
                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
            }
            Instruction::XOR(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::F => u8::from(self.registers.f),
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::D8 => self.read_next_byte(),
                    ArithmeticTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
                };
                let n = self.registers.a ^ value;
                self.registers.f.zero = n == 0;
                self.registers.f.substract = false;
                self.registers.f.half_carry = true;
                self.registers.f.carry = false;
                self.registers.a = n;
                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
            }
            Instruction::CP(target) => {
                let value = match target {
                    ArithmeticTarget::A => self.registers.a,
                    ArithmeticTarget::B => self.registers.b,
                    ArithmeticTarget::C => self.registers.c,
                    ArithmeticTarget::D => self.registers.d,
                    ArithmeticTarget::E => self.registers.e,
                    ArithmeticTarget::F => u8::from(self.registers.f),
                    ArithmeticTarget::H => self.registers.h,
                    ArithmeticTarget::L => self.registers.l,
                    ArithmeticTarget::D8 => self.read_next_byte(),
                    ArithmeticTarget::HLI => self.bus.read_byte(self.registers.get_hl()),
                };

                self.registers.f.zero = self.registers.a == value;
                self.registers.f.substract = true;
                self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);
                self.registers.f.carry = self.registers.a < value;

                match target {
                    ArithmeticTarget::D8 => self.pc.wrapping_add(2),
                    _ => self.pc.wrapping_add(1)
                }
            }
            Instruction::CCF => {
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = !self.registers.f.carry;
                self.pc.wrapping_add(1)
            }
            Instruction::SCF => {
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::RRA => {
                let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
                let new_value = carry_bit | (self.registers.a >> 1);
                self.registers.f.zero = false;
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = self.registers.a & 0b1 == 0b1;
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLA => {
                let carry_bit = if self.registers.f.carry { 1 } else { 0 };
                let new_value = (self.registers.a << 1) | carry_bit;
                self.registers.f.zero = false;
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = (self.registers.a & 0x80) == 0x80;
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RRCA => {
                let new_value = self.registers.a.rotate_right(1);
                self.registers.f.zero = false;
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = self.registers.a & 0b1 == 0b1;
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::RLCA => {
                let carry = (self.registers.a & 0x80) >> 7;
                let new_value = self.registers.a.rotate_left(1) | carry;
                self.registers.f.zero = false;
                self.registers.f.substract = false;
                self.registers.f.half_carry = false;
                self.registers.f.carry = carry == 0x01;
                self.registers.a = new_value;
                self.pc.wrapping_add(1)
            }
            Instruction::CPL => {
                self.registers.a = !self.registers.a;
                self.registers.f.substract = true;
                self.registers.f.half_carry = true;
                self.pc.wrapping_add(1)
            }
            Instruction::DAA => {
                let flags = self.registers.f;
                let mut carry = false;

                let result = if !flags.substract {
                    let mut result = self.registers.a;
                    if flags.carry || self.registers.a > 0x99 {
                        carry = true;
                        result = result.wrapping_add(0x60);
                    }
                    if flags.half_carry || self.registers.a & 0x0F > 0x09 {
                        result = result.wrapping_add(0x06);
                    }
                    result
                } else if flags.carry {
                    carry = true;
                    let add = if flags.half_carry { 0x9A } else { 0xA0 };
                    self.registers.a.wrapping_add(add)
                } else if flags.half_carry {
                    self.registers.a.wrapping_add(0xFA)
                } else {
                    self.registers.a
                };

                self.registers.f.zero = result == 0;
                self.registers.f.half_carry = false;
                self.registers.f.carry = carry;

                self.registers.a = result;
                self.pc.wrapping_add(1)
            }
            Instruction::BIT(target, bit_position) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.bit_test(self.registers.a, bit_position),
                    PreFixTarget::B => self.registers.b = self.bit_test(self.registers.b, bit_position),
                    PreFixTarget::C => self.registers.c = self.bit_test(self.registers.c, bit_position),
                    PreFixTarget::D => self.registers.d = self.bit_test(self.registers.d, bit_position),
                    PreFixTarget::E => self.registers.e = self.bit_test(self.registers.e, bit_position),
                    PreFixTarget::H => self.registers.h = self.bit_test(self.registers.h, bit_position),
                    PreFixTarget::L => self.registers.l = self.bit_test(self.registers.l, bit_position),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.bit_test(value, bit_position);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RES(target, bit_position) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.reset_bit(self.registers.a, bit_position),
                    PreFixTarget::B => self.registers.b = self.reset_bit(self.registers.b, bit_position),
                    PreFixTarget::C => self.registers.c = self.reset_bit(self.registers.c, bit_position),
                    PreFixTarget::D => self.registers.d = self.reset_bit(self.registers.d, bit_position),
                    PreFixTarget::E => self.registers.e = self.reset_bit(self.registers.e, bit_position),
                    PreFixTarget::H => self.registers.h = self.reset_bit(self.registers.h, bit_position),
                    PreFixTarget::L => self.registers.l = self.reset_bit(self.registers.l, bit_position),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.reset_bit(value, bit_position);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SET(target, bit_position) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.set_bit(self.registers.a, bit_position),
                    PreFixTarget::B => self.registers.b = self.set_bit(self.registers.b, bit_position),
                    PreFixTarget::C => self.registers.c = self.set_bit(self.registers.c, bit_position),
                    PreFixTarget::D => self.registers.d = self.set_bit(self.registers.d, bit_position),
                    PreFixTarget::E => self.registers.e = self.set_bit(self.registers.e, bit_position),
                    PreFixTarget::H => self.registers.h = self.set_bit(self.registers.h, bit_position),
                    PreFixTarget::L => self.registers.l = self.set_bit(self.registers.l, bit_position),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.set_bit(value, bit_position);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SRL(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.shift_right_logical(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.shift_right_logical(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.shift_right_logical(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.shift_right_logical(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.shift_right_logical(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.shift_right_logical(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.shift_right_logical(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.shift_right_logical(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RR(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.rotate_right_through_carry_set_zero(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.rotate_right_through_carry_set_zero(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.rotate_right_through_carry_set_zero(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.rotate_right_through_carry_set_zero(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.rotate_right_through_carry_set_zero(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.rotate_right_through_carry_set_zero(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.rotate_right_through_carry_set_zero(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.rotate_right_through_carry_set_zero(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RL(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.rotate_left_through_carry_set_zero(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.rotate_left_through_carry_set_zero(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.rotate_left_through_carry_set_zero(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.rotate_left_through_carry_set_zero(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.rotate_left_through_carry_set_zero(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.rotate_left_through_carry_set_zero(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.rotate_left_through_carry_set_zero(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.rotate_left_through_carry_set_zero(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RRC(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.rotate_right_set_zero(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.rotate_right_set_zero(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.rotate_right_set_zero(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.rotate_right_set_zero(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.rotate_right_set_zero(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.rotate_right_set_zero(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.rotate_right_set_zero(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.rotate_right_set_zero(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::RLC(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.rotate_left_set_zero(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.rotate_left_set_zero(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.rotate_left_set_zero(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.rotate_left_set_zero(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.rotate_left_set_zero(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.rotate_left_set_zero(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.rotate_left_set_zero(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.rotate_left_set_zero(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SRA(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.shift_right_arithmetic(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.shift_right_arithmetic(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.shift_right_arithmetic(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.shift_right_arithmetic(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.shift_right_arithmetic(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.shift_right_arithmetic(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.shift_right_arithmetic(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.shift_right_arithmetic(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SLA(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.shift_left_arithmetic(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.shift_left_arithmetic(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.shift_left_arithmetic(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.shift_left_arithmetic(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.shift_left_arithmetic(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.shift_left_arithmetic(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.shift_left_arithmetic(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.shift_left_arithmetic(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
            }
            Instruction::SWAP(target) => {
                match target {
                    PreFixTarget::A => self.registers.a = self.swap(self.registers.a),
                    PreFixTarget::B => self.registers.b = self.swap(self.registers.b),
                    PreFixTarget::C => self.registers.c = self.swap(self.registers.c),
                    PreFixTarget::D => self.registers.d = self.swap(self.registers.d),
                    PreFixTarget::E => self.registers.e = self.swap(self.registers.e),
                    PreFixTarget::H => self.registers.h = self.swap(self.registers.h),
                    PreFixTarget::L => self.registers.l = self.swap(self.registers.l),
                    PreFixTarget::HLI => {
                        let hl = self.registers.get_hl();
                        let value = self.bus.read_byte(hl);
                        let result = self.swap(value);
                        self.bus.write_byte(hl, result);
                    }
                }
                self.pc.wrapping_add(2)
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
            Instruction::JR(target) => {
                let jump_condition = match target {
                    JumpTest::NotZero => !self.registers.f.zero,
                    JumpTest::NotCarry => !self.registers.f.carry,
                    JumpTest::Zero => self.registers.f.zero,
                    JumpTest::Carry => self.registers.f.carry,
                    JumpTest::Always => true,
                };
                self.jump_rel(jump_condition)
            }
            Instruction::JPI => {
                self.registers.get_hl()
            }
            Instruction::LD(load_type) => {
                match load_type {
                    LoadType::Byte(target, source) => {
                        let source_value = match source {
                            LoadByteSource::A => self.registers.a,
                            LoadByteSource::B => self.registers.b,
                            LoadByteSource::C => self.registers.c,
                            LoadByteSource::D => self.registers.d,
                            LoadByteSource::E => self.registers.e,
                            LoadByteSource::H => self.registers.h,
                            LoadByteSource::L => self.registers.l,
                            LoadByteSource::D8 => self.read_next_byte(),
                            LoadByteSource::HLI => self.bus.read_byte(self.registers.get_hl()),
                        };
                        match target {
                            LoadByteTarget::A => self.registers.a = source_value,
                            LoadByteTarget::B => self.registers.b = source_value,
                            LoadByteTarget::C => self.registers.c = source_value,
                            LoadByteTarget::D => self.registers.d = source_value,
                            LoadByteTarget::E => self.registers.e = source_value,
                            LoadByteTarget::H => self.registers.h = source_value,
                            LoadByteTarget::L => self.registers.l = source_value,
                            LoadByteTarget::HLI => {
                                self.bus.write_byte(self.registers.get_hl(), source_value)
                            }
                        };
                        match source {
                            LoadByteSource::D8 => self.pc.wrapping_add(2),
                            LoadByteSource::HLI => self.pc.wrapping_add(1),
                            _ => self.pc.wrapping_add(1)
                        }
                    }
                    LoadType::Word(target) => {
                        let word = self.read_next_word();
                        match target {
                            LoadWordTarget::BC => self.registers.set_bc(word),
                            LoadWordTarget::DE => self.registers.set_de(word),
                            LoadWordTarget::HL => self.registers.set_hl(word),
                            LoadWordTarget::SP => self.sp = word,
                        };
                        self.pc.wrapping_add(3)
                    }
                    LoadType::AFromIndirect(source) => {
                        self.registers.a = match source {
                            Indirect::BCIndirect => self.bus.read_byte(self.registers.get_bc()),
                            Indirect::DEIndirect => self.bus.read_byte(self.registers.get_de()),
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.read_byte(hl)
                            }
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.read_byte(hl)
                            }
                            Indirect::WordIndirect => self.bus.read_byte(self.read_next_word()),
                            Indirect::LastByteIndirect => {
                                self.bus.read_byte(0xFF00 + self.registers.c as u16)
                            }
                        };

                        match source {
                            Indirect::WordIndirect => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1)
                        }
                    }
                    LoadType::IndirectFromA(target) => {
                        let a = self.registers.a;
                        match target {
                            Indirect::BCIndirect => {
                                let bc = self.registers.get_bc();
                                self.bus.write_byte(bc, a)
                            }
                            Indirect::DEIndirect => {
                                let de = self.registers.get_de();
                                self.bus.write_byte(de, a)
                            }
                            Indirect::HLIndirectMinus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_sub(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::HLIndirectPlus => {
                                let hl = self.registers.get_hl();
                                self.registers.set_hl(hl.wrapping_add(1));
                                self.bus.write_byte(hl, a);
                            }
                            Indirect::WordIndirect => {
                                let word = self.read_next_word();
                                self.bus.write_byte(word, a);
                            }
                            Indirect::LastByteIndirect => {
                                let c = self.registers.c as u16;
                                self.bus.write_byte(0xFF00 + c, a);
                            }
                        };

                        match target {
                            Indirect::WordIndirect => self.pc.wrapping_add(3),
                            _ => self.pc.wrapping_add(1)
                        }
                    }
                    LoadType::ByteAddressFromA => {
                        let offset = self.read_next_byte() as u16;
                        self.bus.write_byte(0xFF00 + offset, self.registers.a);
                        self.pc.wrapping_add(2)
                    }
                    LoadType::AFromByteAddress => {
                        let offset = self.read_next_byte() as u16;
                        self.registers.a = self.bus.read_byte(0xFF00 + offset);
                        self.pc.wrapping_add(2)
                    }
                    LoadType::SPFromHL => {
                        self.sp = self.registers.get_hl();
                        self.pc.wrapping_add(1)
                    }
                    LoadType::IndirectFromSP => {
                        let address = self.read_next_word();
                        let sp = self.sp;
                        self.bus.write_byte(address, (sp & 0xFF) as u8);
                        self.bus
                            .write_byte(address.wrapping_add(1), ((sp & 0xFF00) >> 8) as u8);
                        self.pc.wrapping_add(3)
                    }
                    LoadType::HLFromSPN => {
                        let value = self.read_next_byte() as i8 as i16 as u16;
                        let result = self.sp.wrapping_add(value);
                        self.registers.set_hl(result);
                        self.registers.f.zero = false;
                        self.registers.f.substract = false;
                        self.registers.f.half_carry = (self.sp & 0xF) + (value & 0xF) > 0xF;
                        self.registers.f.carry = (self.sp & 0xFF) + (value & 0xFF) > 0xFF;
                        self.pc.wrapping_add(2)
                    }
                }
            }
            Instruction::PUSH(target) => {
                let value = match target {
                    StackTarget::AF => self.registers.get_af(),
                    StackTarget::BC => self.registers.get_bc(),
                    StackTarget::DE => self.registers.get_de(),
                    StackTarget::HL => self.registers.get_hl(),
                };
                self.push(value);
                self.pc.wrapping_add(1)
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
            Instruction::RST(target) => {
                self.push(self.pc.wrapping_add(1));
                target.to_hex()
            }
            Instruction::NOP => {
                self.pc.wrapping_add(1)
            }
            Instruction::HALT => {
                self.is_halted = true;
                self.pc.wrapping_add(1)
            }
            Instruction::DI => {
                self.pc.wrapping_add(1)
            }
            Instruction::EI => {
                self.pc.wrapping_add(1)
            }
            _ => {
                    self.pc
            }
        }
    }

    fn add(&mut self, n: u8) -> u8 {
        let (value, overflow) = self.registers.a.overflowing_add(n);
        self.registers.f.zero = value == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xf) + (value & 0xf) > 0xf;

        value
    }
    fn add_with_carry(&mut self, n: u8) -> u8 {
        let (add1, carry) = self.registers.a.overflowing_add(n); // A + s
        let (add2, carry2) = add1.overflowing_add(self.registers.f.carry as u8); // A + s + carryflag
        self.registers.f.zero = add2 == 0;
        self.registers.f.substract = false;
        self.registers.f.carry = carry || carry2;
        self.registers.f.half_carry = ((self.registers.a & 0xF) + (n & 0xF) + (self.registers.f.carry as u8)) > 0xF;
        add2
    }
    fn sub(&mut self, n: u8) -> u8 {
        let (value, overflow) = self.registers.a.overflowing_sub(n);
        self.registers.f.zero = value == 0;
        self.registers.f.substract = true;
        self.registers.f.carry = overflow;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (value & 0xF);

        value
    }
    fn sub_with_carry(&mut self, n: u8) -> u8 {
        let (sub, carry) = self.registers.a.overflowing_sub(n);
        let (sub2, carry2) = sub.overflowing_sub(self.registers.f.carry as u8);
        self.registers.f.zero = sub2 == 0;
        self.registers.f.substract = true;
        self.registers.f.carry = carry || carry2;
        self.registers.f.half_carry = (self.registers.a & 0xF) < (n & 0xF) + (self.registers.f.carry as u8);
        sub2
    }
    fn bit_test(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        let result = (value >> bit_position) & 0b1;
        self.registers.f.zero = result == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = true;
        result
    }
    fn reset_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value & !(1 << bit_position)
    }
    fn set_bit(&mut self, value: u8, bit_position: BitPosition) -> u8 {
        let bit_position: u8 = bit_position.into();
        value | (1 << bit_position)
    }
    fn shift_right_logical(&mut self, value: u8) -> u8 {
        let new_value = value >> 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }
    fn rotate_right_through_carry_set_zero(&mut self, value: u8) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 } << 7;
        let new_value = carry_bit | (value >> 1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }
    fn rotate_left_through_carry_set_zero(&mut self, value: u8) -> u8 {
        let carry_bit = if self.registers.f.carry { 1 } else { 0 };
        let new_value = (value << 1) | carry_bit;
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = (value & 0x80) == 0x80;
        new_value
    }
    fn rotate_right_set_zero(&mut self, value: u8) -> u8 {
        let new_value = value.rotate_right(1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }
    fn rotate_left_set_zero(&mut self, value: u8) -> u8 {
        let carry = (value & 0x80) >> 7;
        let new_value = value.rotate_left(1) | carry;
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = carry == 0x01;
        new_value
    }
    fn shift_right_arithmetic(&mut self, value: u8) -> u8 {
        let msb = value & 0x80;
        let new_value = msb | (value >> 1);
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0b1 == 0b1;
        new_value
    }
    fn shift_left_arithmetic(&mut self, value: u8) -> u8 {
        let new_value = value << 1;
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = value & 0x80 == 0x80;
        new_value
    }
    fn swap(&mut self, value: u8) -> u8 {
        let new_value = ((value & 0xf) << 4) | ((value & 0xf0) >> 4);
        self.registers.f.zero = new_value == 0;
        self.registers.f.substract = false;
        self.registers.f.half_carry = false;
        self.registers.f.carry = false;
        new_value
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
    fn jump_rel(&self, should_jump: bool) -> u16 {
        let next_step = self.pc.wrapping_add(2);
        if should_jump {
            let offset = self.read_next_byte() as i8;
            let pc = if offset >= 0 {
                next_step.wrapping_add(offset as u16)
            } else {
                next_step.wrapping_sub(offset.abs() as u16)
            };
            pc
        } else {
            next_step
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

    fn read_next_byte(&self) -> u8 {
        self.bus.read_byte(self.pc + 1)
    }

    pub const fn getRESH() -> usize {
        SCREEN_HEIGHT
    }
    pub const fn getRESW() -> usize {
        SCREEN_WIDTH
    }
}
