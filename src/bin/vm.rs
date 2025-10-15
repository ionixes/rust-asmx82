
use rustmachine::Machine;

pub fn main() -> Result<(), &'static str> {
    let mut vm = Machine::new(5);
    vm.step();
    Ok(())
}

