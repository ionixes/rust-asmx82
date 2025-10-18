
use std::{env, fs::File, path::Path};
use std::io::{self, Write, BufReader, BufRead};



fn main() -> Result<(), String> {
    let args: Vec<_>  = env::args().collect();
    if args.len() != 2 {
        panic!("usage: {} <input>", args[0]);
    }
    // let input_file = args[1];
    let mut output: Vec<u8> = Vec::new();
    let file = File::open(Path::new(&args[1])).map_err(|x| format!("failed to open {}", x))?;
    for line in BufReader::new(file).lines() {
        for word in line.unwrap().split(" ").filter(|x| x.len() > 0) {
            let a: u8 = u8::from_str_radix(word, 16).map_err(|x| format!("parse error: {}", x))?;
            output.push(a);
        }  
    }
    let stdout = io::stdout().write_all(&output).map_err(|_| format!("interrupted error"));
    Ok(())
}
