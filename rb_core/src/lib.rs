
const RAM_SIZE: usize = 8*1024;

struct FlagReg {
    zero: bool,
    substract: bool,
    half_carry: bool,
    carry: bool
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;
struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }
}

impl std::convert::From<FlagReg> for u8 {
    fn from(flag: FlagReg) -> u8 {
        (if flag.zero {1} else {0}) << ZERO_FLAG_BYTE_POSITION |
        (if flag.substract {1} else {0}) << SUBTRACT_FLAG_BYTE_POSITION |
        (if flag.half_carry {1} else {0}) << HALF_CARRY_FLAG_BYTE_POSITION |
        (if flag.carry {1} else {0}) << CARRY_FLAG_BYTE_POSITION
    }
}

impl std::convert::From<u8> for FlagReg {
    fn from(flag: u8) -> FlagReg {
        let zero = (flag >> ZERO_FLAG_BYTE_POSITION) & 0b1 !=0;
        let substract = (flag >> SUBTRACT_FLAG_BYTE_POSITION) & 0b1 !=0;
        let hcarry = (flag >> HALF_CARRY_FLAG_BYTE_POSITION) & 0b1 !=0;
        let carry = (flag >> CARRY_FLAG_BYTE_POSITION) & 0b1 !=0;
        
        FlagReg {
            zero: zero,
            substract: substract,
            half_carry: hcarry,
            carry: carry
        }
    }
}