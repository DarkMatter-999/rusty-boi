#[derive(Copy, Clone)]
pub struct FlagReg {
    pub zero: bool,
    pub substract: bool,
    pub half_carry: bool,
    pub carry: bool
}

impl FlagReg {
    fn new() -> FlagReg {
        FlagReg {
            zero: false,
            substract: false,
            half_carry: false,
            carry: false,
        }
    }
}

const ZERO_FLAG_BYTE_POSITION: u8 = 7;
const SUBTRACT_FLAG_BYTE_POSITION: u8 = 6;
const HALF_CARRY_FLAG_BYTE_POSITION: u8 = 5;
const CARRY_FLAG_BYTE_POSITION: u8 = 4;


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

pub struct Registers {
    pub a: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub f: FlagReg,
    pub h: u8,
    pub l: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: FlagReg::new(),
            h: 0,
            l: 0,
        }
    }
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | u8::from(self.f) as u16
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = (value >> 8) as u8;
        self.f = FlagReg::from((value & 0xff) as u8);
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = (value >> 8) as u8;
        self.c = (value & 0xff) as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = (value >> 8) as u8;
        self.e = (value & 0xff) as u8;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = (value >> 8) as u8;
        self.l = (value & 0xff) as u8;
    }
}
