use std::env;
use std::process;

use alu::{/*Instruction,*/ Core};


fn main() {
    println!("AVR-like Instruction Set simulator");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        panic!("Error, \nname binary_file_path");
    }
    let file_path = args[1].clone();
    let mut core = Core::new();
    
    core.init_system(String::from(file_path));


    if let Err(e) = core.run_program(){
        println!("Unexpected error: {}", e);
        process::exit(1);
    }
    core.dump_memory();

}
