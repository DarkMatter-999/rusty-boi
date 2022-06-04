
pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
    RLC(PreFixTarget),
    JP(JumpTest),
    LD(LoadType),
    PUSH(StackTarget),
    POP(StackTarget),
    CALL(JumpTest),
    RET(JumpTest),
    NOP,
    HALT,
    
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