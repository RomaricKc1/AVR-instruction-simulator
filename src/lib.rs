use std::error::Error;
use std::collections::HashMap;
use std::process;
use std::i16;

pub struct Instruction{
    pub reg_mode: String,
    pub regs: HashMap<String, Register>,
    pub op: String,
}

pub struct Register{
    pub name: String,
    pub value: i16,
}

impl Register{
    pub fn new(name: &str, value:i16) -> Result<Register, &str>{
        Ok( Register { name: name.to_string(), value } )
    }
}

impl Instruction{
    pub fn new(args: &[String]) -> Result<Instruction, &str>{
        if args.len() < 7{
            panic!("Lol, I am not going to accept that");
        }
        let reg_mode = args[1].clone();

        let r1 = args[2].clone();
        let r1_val: i16 = i16::from_str_radix((args[3].clone()).trim_start_matches("0x"), 16).expect("expected an Hex value");

        let r2 = args[4].clone();
        let r2_val: i16 = i16::from_str_radix((args[5].clone()).trim_start_matches("0x"), 16).expect("expected an Hex value");

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
    
    
    pub fn fetch(instruction:Instruction) -> Result<(), Box<dyn Error>>{
        match instruction.reg_mode.as_str() {
            "Register_Direct" => {
                let op = instruction.op;

                let reg_src = instruction.regs.get("src").unwrap_or_else(||{
                    println!("error");
                    process::exit(1);
                });
                let reg_dest = instruction.regs.get("dest").unwrap_or_else(||{
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

    pub fn execute_2(op: String, reg_dest: &mut Register, reg_src: &Register) -> Register{
        println!("{} {}", reg_src.value, reg_dest.value);
        
        match op.as_str() {
            "MOV"   => reg_dest.value =  reg_src.value,
            "OR"    => reg_dest.value |=  reg_src.value,
            "AND"   => reg_dest.value &=  reg_src.value,
            _ => println!("Undefined instruction")
        }
        Register { name: reg_dest.name.to_string(), value: reg_dest.value }
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn or_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x01_FF,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x01_4F,
        };
        let op = String::from("OR");
        let result = Instruction::execute_2(op, &mut r2, &r1);
        assert_eq!(0x01_FF, result.value);
    }
    #[test] 
    fn and_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x01_2f,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x01_44,
        };
        let op = String::from("AND");
        let result = Instruction::execute_2(op, &mut r2, &r1);
        assert_eq!(0x01_04, result.value);
    }
    #[test]
    #[should_panic]
    fn or_confused_with_add_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x01_FF,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x11_4F,
        };
        let op = String::from("OR");
        
        let result = Instruction::execute_2(op, &mut r2, &r1);

        assert_eq!(0x03_4E, result.value);
    }
     
}