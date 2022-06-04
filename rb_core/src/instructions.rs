
pub enum Instruction {
    // Arithmetic Instructions
    INC(IncDecTarget),
    DEC(IncDecTarget),

    ADD(ArithmeticTarget),
    ADC(ArithmeticTarget),
    ADDHL(ADDHLTarget),
    ADDSP,
    SUB(ArithmeticTarget),
    SBC(ArithmeticTarget),
    AND(ArithmeticTarget),
    OR(ArithmeticTarget),
    XOR(ArithmeticTarget),
    CP(ArithmeticTarget),

    CCF,
    SCF,

    RRA,
    RLA,
    RRCA,
    RLCA,
    CPL,
    DAA,

    // Prefix Instructions
    BIT(PreFixTarget, BitPosition),
    RES(PreFixTarget, BitPosition),
    SET(PreFixTarget, BitPosition),
    SRL(PreFixTarget),
    RR(PreFixTarget),
    RL(PreFixTarget),
    RRC(PreFixTarget),
    RLC(PreFixTarget),
    SRA(PreFixTarget),
    SLA(PreFixTarget),
    SWAP(PreFixTarget),

    // Jump Instructions
    JP(JumpTest),
    JR(JumpTest),
    JPI,

    // Load Instructions
    LD(LoadType),

    // Stack Instructions
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    RETI,
    RST(RSTLocation),

    // Control Instructions
    HALT,
    NOP,
    DI,
    EI,
}

impl Instruction {
    pub fn from_byte(byte: u8, prefixed: bool) -> Option<Instruction> {
        if prefixed {
            Instruction::from_byte_prefixed(byte)
        } else {
            Instruction::from_byte_not_prefixed(byte)
        }
    }

    fn from_byte_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x00 => Some(Instruction::RLC(PreFixTarget::B)),
            _ => None,
        }
    }

    fn from_byte_not_prefixed(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::DE)),
            _ => None,
        }
    }
}

pub enum ArithmeticTarget {
    A,B,C,D,E,F,H,L,
}

pub enum IncDecTarget {
    A,B,C,D,E,F,H,L,HLI,BC,DE,HL,SP,
}

pub enum PreFixTarget {
    A,B,C,D,E,H,L,HLI,
}

pub enum JumpTest {
    NotZero,Zero,NotCarry,Carry,Always
}

pub enum LoadByteTarget {
    A,B,C,D,E,H,L,HLI,
}

pub enum LoadByteSource {
    A,B,C,D,E,H,L,HLI,D8,
}

pub enum LoadType {
    Byte(LoadByteTarget, LoadByteSource)
}

pub enum StackTarget {
    AF,BC,DE,HL,
}

pub enum ADDHLTarget {
    BC,DE,HL,SP,
}

pub enum BitPosition {
    B0,B1,B2,B3,B4,B5,B6,B7,
}
impl std::convert::From<BitPosition> for u8 {
    fn from(position: BitPosition) -> u8 {
        match position {
            BitPosition::B0 => 0,
            BitPosition::B1 => 1,
            BitPosition::B2 => 2,
            BitPosition::B3 => 3,
            BitPosition::B4 => 4,
            BitPosition::B5 => 5,
            BitPosition::B6 => 6,
            BitPosition::B7 => 7,
        }
    }
}

pub enum RSTLocation {
    x00,x08,x10,x18,x20,x28,x30,x38,
}

impl RSTLocation {
    pub fn to_hex(&self) -> u16 {
        match self {
            RSTLocation::x00 => 0x00,
            RSTLocation::x08 => 0x08,
            RSTLocation::x10 => 0x10,
            RSTLocation::x18 => 0x18,
            RSTLocation::x20 => 0x20,
            RSTLocation::x28 => 0x28,
            RSTLocation::x30 => 0x30,
            RSTLocation::x38 => 0x38,
        }
    }
}