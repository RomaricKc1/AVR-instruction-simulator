use std::env;
use std::process;

use alu::Instruction;

fn main() {
    println!("AVR-8 I simulatior");

    let args: Vec<String> = env::args().collect();

    let addr_mode = Instruction::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args, {}", err);
        process::exit(1);
    });

    if let Err(e) = Instruction::fetch(addr_mode){
        println!("Unexpected error lol {}", e);
        process::exit(1);
    }

}
