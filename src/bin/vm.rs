
use std::{env, fs::File, io::{BufReader, Read}, path::Path};

use rustmachine::{Machine, Register};

fn signal_halt(vm: &mut Machine) -> Result<(), String> {
    vm.halt = true;
    Ok(())
}

pub fn main() -> Result<(), String> {

    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }

    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open {}", x))?;
    let mut prog: Vec<u8> = Vec::new();
    let mut reader = BufReader::new(file);
    let _ = reader.read_to_end(&mut prog);

    let mut vm = Machine::new(5);

    vm.define_handler(0x90, signal_halt);

    let sc = vm.memory.from_vector(prog, 0).unwrap();
    while !vm.halt {
        vm.step()?;
    }
    println!("A = {}", vm.get_reg(Register::A));
    Ok(())
}
