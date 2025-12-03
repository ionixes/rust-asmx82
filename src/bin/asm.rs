
use std::{env, fs::File, path::Path};
use std::io::{self, Write, BufReader, BufRead};

use rustmachine::{Op, OpFL, Register};



/* TODO Change the syntax more similiar to reality assembler
 * TEST ASM
 * push 10
 * push 08
 * pop a
 * pop c
 * add a, c
 * sig 144
 *
 * */

fn parse_numeric(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s, 10).map_err(|_| format!("parse number err: {:?}", s))
}

fn assert_length(parts: &Vec<&str>, n: usize) -> Result<(), String> {
    if parts.len() == n {
        Ok(())
    } else {
        Err(format!("expected {} got {}", parts.len(), n))
    }
}

fn handle_code(parts: Vec<&str>) -> Result<Op, String> {
        let code = OpFL::from_str(parts[0]).ok_or(format!("the instruction {:?} is unknown", parts[0])).unwrap();
        match code {
            OpFL::Nop => {
                Ok(Op::Nop)
            },
            OpFL::Push => {
                assert_length(&parts, 2)?;
                Ok(Op::Push(parse_numeric(parts[1]).unwrap()))
            },
            OpFL::PopReg => {
                assert_length(&parts, 2)?;
                Ok(Op::PopReg(Register::from_u8(parse_numeric(parts[1]).unwrap()).ok_or(format!("not found this register")).unwrap()))
            },
            OpFL::AddStack => {
                Ok(Op::AddStack)
            },
            OpFL::SubStack => {
                Ok(Op::SubStack)
            },
            OpFL::AddReg => {
                assert_length(&parts, 2)?;
                let reg  = parse_numeric(parts[1]).unwrap();
                let reg1 = (reg & 0xf0) >> 4;
                let reg2 = reg & 0x0f;
//                println!("{:04b} --- {:04b}", reg1, reg2);
                Ok(Op::AddReg(Register::from_u8(reg1)
                    .ok_or(format!("unknown register {:#02x}", reg1)).unwrap(),
                          Register::from_u8(reg2)
                    .ok_or(format!("unknown register {:#02x}", reg2)).unwrap()))

            },
            OpFL::Signal => {
                assert_length(&parts, 2)?;
                Ok(Op::Signal(parse_numeric(parts[1]).unwrap()))
            }
            _ => Err(format!("assembler error: unknown code {:?}", parts[0]))
        }
} 

fn main() -> Result<(), String> {
    let args: Vec<_>  = env::args().collect();
    if args.len() != 2 {
        // TODO: make help
        panic!("usage: {} <input>", args[0]);
    }
    // let input_file = args[1];
    let mut output: Vec<u8> = Vec::new();
    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open {}", x))?;
    for line in BufReader::new(file).lines().filter(|x| x.as_ref().unwrap().len() > 0) {
        let line_inner = line.map_err(|_x| "error during read the asm file")?;
        let parts: Vec<&str> = line_inner.split(" ").filter(|x| x.len() > 0).collect();
        if parts[0].to_string() == ";" {
            continue;
        }
        // println!("{:?}", parts);
        let instruction = handle_code(parts)?;
        let raw_instruction: u16 = instruction.encode_u16()
                    .ok_or(format!("can't encode the instruction")).unwrap();
        //println!("{:#04x}", raw_instruction);
        output.push((raw_instruction&0xff) as u8);
        output.push((raw_instruction>>8) as u8);
    }
    let stdout = io::stdout().write_all(&output).map_err(|_| format!("interrupted error"));
    Ok(())
}
