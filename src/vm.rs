
use std::collections::HashMap;
#[allow(dead_code)]

use std::usize;

use crate::memory::*;
use crate::instruction::*;

fn parse_incstruction(oparg: u16) -> Result<Op, String> {
    let op = (oparg & 0xff) as u8;
    match op {
        x if x == OpFL::Nop as u8 => Ok(Op::Nop),
        x if x == OpFL::Push as u8 => {
            let arg = (oparg & 0xf00) >> 8;
            Ok(Op::Push(arg as u8))
        },
        x if x == OpFL::PopReg as u8 => {
            let reg = ((oparg & 0xf00) >> 8) as u8;
            Register::from_u8(reg)
                .ok_or(format!("unknown register {:#02x}", reg))
                .map(|r| Op::PopReg(r))
        },
        x if x == OpFL::AddStack as u8 => Ok(Op::AddStack),
        x if x == OpFL::SubStack as u8 => Ok(Op::SubStack),
        x if x == OpFL::AddReg as u8 => {
            let reg = ((oparg & 0xf00) >> 8) as u8;
            let reg1 = (reg & 0xf0) >> 4;
            let reg2 = reg & 0x0f;
            Ok(Op::AddReg(Register::from_u8(reg1)
                    .ok_or(format!("unknown register {:#02x}", reg1)).unwrap(),
                          Register::from_u8(reg2)
                    .ok_or(format!("unknown register {:#02x}", reg2)).unwrap()))
        },
        x if x == OpFL::Signal as u8 => {
            let arg = (oparg & 0xff00) >> 8;
            Ok(Op::Signal(arg as u8))
        },
        _ => Ok(Op::Unknown(op)),
    }
}

pub type SignalFunction = fn(&mut Machine) -> Result<(), String>;

pub struct Machine {
    registers: [u16; 8],
    signal_handlers: HashMap<u8, SignalFunction>,
    pub halt: bool,
    pub memory: Box<dyn Addressable>,
}


impl Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 8],
            signal_handlers: HashMap::new(),
            halt: false,
            memory: Box::new(LinearMemory::new(memory_size * 1024)),
        }
    }

    pub fn get_reg(&self, r: Register) -> u16 {
        self.registers[r as usize]
    }

    pub fn define_handler(&mut self, index: u8, f: SignalFunction) {
        self.signal_handlers.insert(index, f);
    }

    pub fn push(&mut self, v: u16) -> Result<(), String> {
        let sp = self.registers[Register::SP as usize];
        if !self.memory.write2(sp, v) {
            return Err(format!("memory write fault @ {:#02x}", sp));
        }
        self.registers[Register::SP as usize] += 2;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<u16, String> {
        let sp = self.registers[Register::SP as usize] - 2;
        if let Some(s) = self.memory.read2(sp) {
            Ok(s)
        } else {
            Err(format!("memory fault @ {:#02x}", sp))
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).ok_or(format!("fault read memory @ {:#02x}", pc)).unwrap();
        self.registers[Register::PC as usize] = pc + 2;
        let op: Op = parse_incstruction(instruction)?;
        match op {
            Op::Nop => Ok(()),
            Op::Push(v) => self.push(v as u16),
            Op::PopReg(r) => {
                let s = self.pop()?;
                self.registers[r as usize] = s;
                self.registers[Register::SP as usize] -= 2;
                Ok(())
            },
            Op::AddStack => {
                // DONE: make pop by operator
                /*
                self.memory.write(pc + 2, 0x2);
                self.memory.write(pc + 3, 0x0);
                self.memory.write(pc + 4, 0x2);
                self.memory.write(pc + 5, 0x1);

                self.step()?;
                self.step()?;
                self.step()?;
                self.step()?;

                 * */
                let v1 = self.pop()?;
                self.registers[Register::SP as usize] -= 2;
                let v2 = self.pop()?;
                self.registers[Register::SP as usize] -= 2;
                self.push(v1 + v2)
            },
            Op::SubStack => {
                let v1 = self.pop()?;
                self.registers[Register::SP as usize] -= 2;
                let v2 = self.pop()?;
                self.registers[Register::SP as usize] -= 2;

                self.push(v2 - v1)

            },
            Op::AddReg(r1, r2) => {
                self.registers[r1 as usize] += self.registers[r2 as usize];
                Ok(())
            },
            Op::Signal(signal) => {
                let sh = self.signal_handlers.get(&signal)
                                    .ok_or(format!("unknown signal {:#02x}", signal)).unwrap();
                sh(self)
            },
            _ => Err(format!(
                "unknown operation {:#02x} @ {:#02x}",
                op.to_u8().unwrap(),
                pc
            )),
        }
    }
}
