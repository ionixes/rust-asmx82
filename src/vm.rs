
use crate::memory::*;


pub enum Register {
    A, B, C,  M, SP, PC, BP, FLAGS,
}



pub struct Machine {
    registers: [u16; 8],
    memory: Box<dyn Addressable>
}

impl Machine {
    pub fn new(memory_size: usize) -> Self {
        Self {
            registers: [0; 8],
            memory: Box::new(LinearMemory::new(memory_size*1024))
        }
    }
    pub fn step(&mut self) -> Result<(), &'static str> {
        let pc = self.registers[Register::PC as usize];
        let instruction = self.memory.read2(pc).unwrap();
        println!("{} @ {}", instruction, pc);
        Ok(())
    }
}
