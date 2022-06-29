use std::env;
use std::process;
use std::error::Error;
use std::collections::HashMap;

struct Instruction{
    reg_mode: String,
    regs: HashMap<String, Register>,
    op: String,
}

struct Register{
    name: String,
    value: i16,
}

impl Register{
    fn new(name: &str, value:i16) -> Result<Register, &str>{
        Ok( Register { name: name.to_string(), value } )
    }
}

impl Instruction{
    fn new(args: &[String]) -> Result<Instruction, &str>{
        if args.len() < 7{
            panic!("Lol, I am not going to accept that");
        }
        let reg_mode = args[1].clone();

        let r1 = args[2].clone(); 
        let r1_val:i16 = (args[3].clone()).trim().parse().expect("Expected a number");

        let r2 = args[4].clone(); 
        let r2_val:i16 = (args[5].clone()).trim().parse().expect("Expected a number");

        let operation = args[6].clone();

        let reg_1= Register::new(&r1, r1_val).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });

        let reg_2= Register::new(&r2, r2_val).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
 
        let mut regs: HashMap<String, Register> = HashMap::new();

        regs.insert("src".to_string(), reg_1);
        regs.insert(String::from("dest"), reg_2);

        Ok( Instruction { reg_mode, regs, op: operation })
    }
    
    
    fn fetch(addr_mode:Instruction) -> Result<(), Box<dyn Error>>{
        match addr_mode.reg_mode.as_str() {
            "Register_Direct" => {
                let op = addr_mode.op;

                let reg_src = addr_mode.regs.get("src").unwrap_or_else(||{
                    println!("error");
                    process::exit(1);
                });
                let reg_dest = addr_mode.regs.get("dest").unwrap_or_else(||{
                    println!("error");
                    process::exit(1);
                });

                println!("################################\nExecution mode: Register Direct");
                println!("Intrutcion execution\nOp Code \t\t reg src\t\t reg dest\n---------------------------------------------------------------");
                
                // actual call to execute
                let mut rd = Register { name: reg_dest.name.to_string(), value: reg_dest.value };
                let reg_result = Instruction::execute_2(op.clone(), &mut rd, reg_src);
                
                println!("{} \t\t {}[value: {}] \t\t {}[value: {}, new value: {}] \t\t", 
                    op, reg_src.name, reg_src.value, reg_dest.name, reg_dest.value,
                    reg_result.value);
               
                
            },
            _ => println!("Not implemented, Sorry")
        };
        
        
        Ok(())
    }

    fn execute_2(op: String, reg_dest: &mut Register, reg_src: &Register) -> Register{
        // let val: i16 = 0x69;
        
        match op.as_str() {
            "MOV"   => reg_dest.value =  reg_src.value,
            "OR"    => reg_dest.value |=  reg_src.value,
            "AND"   => reg_dest.value &=  reg_src.value,
            _ => println!("Undefined instruction")
        }
        Register { name: reg_dest.name.to_string(), value: reg_dest.value }
    }
}



fn main() {
    println!("AVR-8 I simulatior");

    let args: Vec<String> = env::args().collect();

    let addr_mode = Instruction::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing args, {}", err);
        process::exit(1);
    });

    //println!("Addressing register mode: {}, involving {} registers", addr_mode.reg_mode, addr_mode.regs_num-1);

    if let Err(e) = Instruction::fetch(addr_mode){
        println!("Unexpected error lol {}", e);
        process::exit(1);
    }








}
