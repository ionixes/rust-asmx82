
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Register {
    A,
    B,
    C,
    M,
    SP,
    PC,
    BP,
    FLAGS,
}

impl Register {
    pub fn from_u8(v: u8) -> Option<Register> {
        match v {
            x if x == Register::A as u8 => Some(Register::A),
            x if x == Register::B as u8 => Some(Register::B),
            x if x == Register::C as u8 => Some(Register::C),
            x if x == Register::M as u8 => Some(Register::M),
            x if x == Register::SP as u8 => Some(Register::SP),
            x if x == Register::PC as u8 => Some(Register::PC),
            x if x == Register::BP as u8 => Some(Register::BP),
            x if x == Register::FLAGS as u8 => Some(Register::FLAGS),
            _ => None
        }
    }
}



#[repr(u8)]
pub enum OpFL {
    Nop,
    Push,
    PopReg,
    AddStack,
    SubStack,
    AddReg,
    Signal
//    Unknown,
}

impl OpFL {
    pub fn from_str(part: &str) -> Option<Self> {
        match part {
            "nop" => Some(Self::Nop),
            "push" => Some(Self::Push),
            "pop" => Some(Self::PopReg),
            "add" => Some(Self::AddStack),
            "sub" => Some(Self::SubStack),
            "adr" => Some(Self::AddReg),
            "sig" => Some(Self::Signal),
            _ => None
        }
    }
}

pub enum Op {
    Nop,
    Push(u8),
    PopReg(Register),
    AddStack,
    SubStack,
    AddReg(Register, Register),
    Signal(u8),
    Unknown(u8),
}

// TODO: make a struct Instruction that present an instruction

impl Op {
    pub fn to_u8(&self) -> Result<u8, String> {
        match self {
            Op::Nop => Ok(OpFL::Nop as u8),
            Op::Push(_) => Ok(OpFL::Push as u8),
            Op::PopReg(_) => Ok(OpFL::PopReg as u8),
            Op::AddStack => Ok(OpFL::AddStack as u8),
            Op::SubStack => Ok(OpFL::SubStack as u8),
            Op::AddReg(_, _) => Ok(OpFL::AddReg as u8),
            Op::Signal(_) => Ok(OpFL::Signal as u8),
            Op::Unknown(x) => Err(format!("unknown operator {:#02x}", x)),
        }
    }
    pub fn encode_u16(&self) -> Option<u16> {
        match self {
        Op::Nop => Some(OpFL::Nop as u16),
        Op::Push(x) => Some((OpFL::Push as u16 | ((*x as u16) << 8)) as u16),
        Op::PopReg(x) => Some((OpFL::PopReg as u16 | (((*x as u16) & 0xf) << 8)) as u16 ),
        Op::AddStack => Some(OpFL::AddStack as u16),
        Op::SubStack => Some(OpFL::SubStack as u16),
        Op::AddReg(x, y) => {
//            println!("{}", ((OpFL::AddReg as u16) << 8) | (((*x as u8) << 4) | (*y as u8)) as u16);
            Some((OpFL::AddReg as u16) | (*x as u16) << 6 | (*y as u16) << 8)
        },
        Op::Signal(x) => Some((OpFL::Signal as u16 | ((*x as u16) << 8)) as u16),
        _ => None
        }
    }
}

