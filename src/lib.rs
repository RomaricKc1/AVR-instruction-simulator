use std::error::Error;
use std::collections::HashMap;
use std::process;
use std::i16;
use std::vec;


/*
 * 16 registers available
 * instruction coded in 16 bit
 * id: 0000 0000 0000 0000 : instruction nop


*/
pub struct Instruction{
    pub reg_mode: String,
    pub regs: HashMap<String, Register>,
    pub op: String,
}

pub struct InstructionSet {
    pub isa: HashMap<String, u8>,
}

pub struct MemoryMap {
    pub register: HashMap<String, u32>, // general purpose registers
    pub stack: Vec<Register>,// stack, iterator of 16 * u32 of size
    pub memory: HashMap<String, u32>, // memory of the cpu, 
}


pub struct Register{
    pub name: String,
    pub value: i16,
}

/*
 * Fetch Decode Execute
 */
pub struct Core {
    pub memory_map: MemoryMap,
    pub program: HashMap<u8, Vec<u8>>,
    pub pc:u32,
}

impl InstructionSet {
    pub fn new() -> InstructionSet{
        let mut isa = HashMap::new();

        isa.insert(String::from("NOP "), 0);
        isa.insert(String::from("ADD"), 1);
        isa.insert(String::from("SUB"), 2);
        isa.insert(String::from("MOV"), 3);
        isa.insert(String::from("PUSH"), 4);

        // ...

        InstructionSet { isa }
    }
}

impl Core {
    pub fn new(&mut self) -> Core{
        let mut memory_map = MemoryMap::new();
        let mut program = HashMap::new();

        let instruction_to_exec = vec![/*op code*/0, 0, 0, 1,/*reg dest*/ 0, 0, 0, 1, /*reg src*/ 0, 0, 0, 1, /*unused*/ 0, 0, 0, 0, 0];
        program.insert(0, instruction_to_exec); // instruction to execute : add r0,r1

        let mut pc:u32 = 0;
        Core{memory_map, program, pc/*, data*/}
    }

    pub fn fetch_instruction(&mut self, number:u8) -> &Vec<u8>{
        // fetch already know the pc, so it grab the instruction machine code from at address pc
        // get the instruction
        let instruction_read = self.program.get(&number).unwrap_or_else(||{
            println!("error");
            process::exit(1);
        });
        instruction_read
    }

    pub fn decode_instruction(instruction_to_decode:&Vec<u8>) -> Vec<String>{
        // now parse the instruction to read the register source and distination and also the operation to do
        // instruction contains OP REG1 REG 2 UNUSED*2 everything is coded in 1 byte
        // the first 4 bits are the op code
        // 0000 0000 0000 0000 0000 : instruction nop
        let opcode = &instruction_to_decode[0..4]; // returns 0, 1, 2, 3 th element
        let reg_src = &instruction_to_decode[4..8];
        let reg_dest = &instruction_to_decode[8..12];

        let mut op_human = String::new();
        let mut reg_src_human = String::new();
        let mut reg_dest_human = String::new();

        // now convert the vector into a binary number
        let mut op_value:u8 = 0;
        for elm in opcode{
            let mut i=3;
            op_value = op_value + elm * 2^i;
            i = i-1;
        }
        let mut reg_src_value:u8 = 0;
        for elm in reg_src{
            let mut i=3;
            reg_src_value = reg_src_value + elm * 2^i;
            i = i-1;
        }
        let mut reg_dest_value:u8 = 0;
        for elm in reg_dest{
            let mut i=3;
            reg_dest_value = reg_dest_value + elm * 2^i;
            i = i-1;
        }
        // non exhaustive
        match op_value {
            0x00 => op_human.push_str("NOP"),
            0x01 => op_human.push_str("ADD"),
            0x02 => op_human.push_str("SUB"),
            0x03 => op_human.push_str("MOV"),
            _    => op_human.push_str(" "),
        };

        match reg_src_value {
            0x01 => reg_src_human.push_str("r0"),
            0x02 => reg_src_human.push_str("r1"),
            0x03 => reg_src_human.push_str("r2"),
            0x04 => reg_src_human.push_str("r3"),
            _    => reg_src_human.push_str(" "),
        };

        match reg_dest_value {
            0x01 => reg_dest_human.push_str("r0"),
            0x02 => reg_dest_human.push_str("r1"),
            0x03 => reg_dest_human.push_str("r2"),
            0x04 => reg_dest_human.push_str("r3"),
            _    => reg_dest_human.push_str(" "),
        };

        // everything set, now return the machin code as human readable
        let readable = vec![op_human, reg_src_human, reg_dest_human];

        readable
    }

    pub fn execute(instruction_human:Vec<String>) {
        // now we execute the instruction with the old function
        // we start by reading the registers value and store them locally
        // aka register source and destination. 
        // we should provide the memory map here as input?? because the data are stored there


        // read register source data


        //read register destination data


        // write register detination with the result, the memory map should be mutable



    }
}
/*
 * 6 registers r0 to r5
 * SP: Stack pointer, PC: Program counter, LR: Link Register
*/
impl MemoryMap {
    #[allow(non_snake_case)]
    pub fn new() -> MemoryMap{

        // [register address at memory and their value]
        // register r0 to r7
        let r0= Register::new("r0", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r1= Register::new("r1", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r2= Register::new("r2", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r3= Register::new("r3", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r4= Register::new("r4", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r5= Register::new("r5", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r6= Register::new("r6", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });

        // register r7 to r12
        let r7= Register::new("r7", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r8= Register::new("r8", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r9= Register::new("r9", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r10= Register::new("r10", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r11= Register::new("r11", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let r12= Register::new("r12", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        
        // special registers
        let SP= Register::new("SP", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let LR= Register::new("LR", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });
        let PC= Register::new("PC", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });

        let mut hashmap: HashMap<String, u32> = HashMap::new();
        let mut stack = Vec::new();
        let pc_temp=Register{value:PC.value, name:String::from("PC")};

        // [memory program and datayy]
        // memory is of 16x32 bit or 512 bit or 64 Bytes or 0.064 kilobyte haha
        let mut memory: HashMap<String, u32> = HashMap::new();

        // program memory
        memory.insert(String::from("0x00_00"), 0x00_00); // at address 0x00_00, memory contains 0
        memory.insert(String::from("0x00_01"), 0x00_00); // at address 0x00_01, memory contains 0
        memory.insert(String::from("0x00_02"), 0x00_00); // at address 0x00_02, memory contains 0
        memory.insert(String::from("0x00_03"), 0x00_00); // at address 0x00_03, memory contains 0
        
        // data memory
        memory.insert(String::from("0x00_04"), 0x00_00); // at address 0x00_04, memory contains 0
        memory.insert(String::from("0x00_05"), 0x00_00); // at address 0x00_05, memory contains 0
        memory.insert(String::from("0x00_06"), 0x00_00); // at address 0x00_06, memory contains 0
        memory.insert(String::from("0x00_07"), 0x00_00); // at address 0x00_07, memory contains 0
        memory.insert(String::from("0x00_08"), 0x00_00); // at address 0x00_08, memory contains 0
        memory.insert(String::from("0x00_09"), 0x00_00); // at address 0x00_09, memory contains 
        
        memory.insert(String::from("0x00_0A"), 0x00_00); // at address 0x00_0A, memory contains 0
        memory.insert(String::from("0x00_0B"), 0x00_00); // at address 0x00_0B, memory contains 0
        memory.insert(String::from("0x00_0C"), 0x00_00); // at address 0x00_0C, memory contains 0
        memory.insert(String::from("0x00_0D"), 0x00_00); // at address 0x00_0D, memory contains 0
        memory.insert(String::from("0x00_0E"), 0x00_00); // at address 0x00_0E, memory contains 0
        memory.insert(String::from("0x00_0F"), 0x00_00); // at address 0x00_0F, memory contains 0

        stack.push(pc_temp);

        //[map the register into the memory]
        hashmap.insert(r0.name, 0x20_00);
        hashmap.insert(r1.name, 0x20_01);
        hashmap.insert(r2.name, 0x20_02);
        hashmap.insert(r3.name, 0x20_03);
        hashmap.insert(r4.name, 0x20_04);
        hashmap.insert(r5.name, 0x20_05);
        hashmap.insert(r6.name, 0x20_06);
        hashmap.insert(r7.name, 0x20_07);
        hashmap.insert(r8.name, 0x20_08);
        hashmap.insert(r9.name, 0x20_09);
        hashmap.insert(r10.name, 0x20_0A);
        hashmap.insert(r11.name, 0x20_0B);
        hashmap.insert(r12.name, 0x20_0C);

        hashmap.insert(SP.name, 0x20_0D);
        hashmap.insert(LR.name, 0x20_0E);
        hashmap.insert(PC.name, 0x20_0F);

        MemoryMap { register:hashmap, stack, memory }
    }
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
