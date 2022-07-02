use std::error::Error;
use std::collections::HashMap;
use std::process;
use std::{u8, /*i16*/};
use std::vec;
use std::mem::transmute;


/*
 * 16 registers available
 * instruction coded in 16 bit
 * id: 0000 0000 0000 0000 0000 : instruction nop


*/
pub struct Instruction{
    pub reg_mode: String,
    pub regs: HashMap<String, Register>,
    pub op: String,
}

pub struct MemoryMap {
    pub register: HashMap<String, Register>, // general purpose registers
    pub stack: Vec<Register>,// stack, iterator of 16 * u32 of size
    pub memory: HashMap<u8, u32>, // memory of the cpu, 
}


pub struct Register{
    pub name: String,
    pub value: u8,
}

/*
 * Fetch Decode Execute
 */
pub struct Core {
    pub memory_map: MemoryMap,
    pub program: HashMap<u8, Vec<u8>>,
    pub pc:u8,
}

impl Core {
    pub fn new() -> Core{
        let memory_map = MemoryMap::new();
        let mut program = HashMap::new();

        //vec![/*op code*/0, 0, 0, 1,/*reg dest*/ 0, 0, 0, 1, /*reg src*/ 0, 0, 0, 1, /*unused*/ 0, 0, 0, 0];
        let instruction_to_exec = vec![];
        program.insert(0, instruction_to_exec); // instruction to execute : add r0,r1

        let pc:u8 = 0;
        Core{memory_map, program, pc/*, data*/}
    }

    pub fn fetch_instruction(&mut self) -> &Vec<u8>{
        // fetch already know the pc, so it grab the instruction machine code from at address pc
        // get the instruction
        let instruction_read = self.program.get(&self.pc).unwrap_or_else(||{
            println!("error");
            process::exit(1);
        });
        self.pc += 1; // increment the pc
        instruction_read
    }

    pub fn decode_instruction(self, instruction_to_decode:&Vec<u8>) -> Vec<String>{
        // now parse the instruction to read the register source and distination and also the operation to do
        // instruction contains OP REG1 REG 2 UNUSED*2 everything is coded in 1 byte
        // the first 4 bits are the op code
        // 0000 0000 0000 0000 : instruction nop
        let opcode = &instruction_to_decode[0..4]; // returns 0, 1, 2, 3 th element
        let reg_src = &instruction_to_decode[4..8];
        let reg_dest = &instruction_to_decode[8..12];

        let mut op_human = String::new();
        let mut reg_src_human = String::new();
        let mut reg_dest_human = String::new();

        // now convert the vector into a binary number
        let op_value = vect_bin_to_dec(opcode);
        let reg_dest_value = vect_bin_to_dec(reg_dest);
        let reg_src_value = vect_bin_to_dec(reg_src);

        match op_value {
            0x00 => op_human.push_str("NOP"),
            0x01 => op_human.push_str("ADD"),
            0x02 => op_human.push_str("SUB"),
            0x03 => op_human.push_str("OR"),
            0x04 => op_human.push_str("AND"),

            0x08 => op_human.push_str("LDI"),
            0x09 => op_human.push_str("LDS"),
            0x0A => op_human.push_str("MOV"),
            
            _    => op_human.push_str(" "),
        };

        match reg_src_value {
            0x01 => reg_src_human.push_str("r0"),
            0x02 => reg_src_human.push_str("r1"),
            0x03 => reg_src_human.push_str("r2"),
            0x04 => reg_src_human.push_str("r3"),

            0x05 => reg_src_human.push_str("r4"),
            0x06 => reg_src_human.push_str("r5"),
            0x07 => reg_src_human.push_str("r6"),
            0x08 => reg_src_human.push_str("r7"),
            0x09 => reg_src_human.push_str("r8"),

            0x0A => reg_src_human.push_str("r9"),
            0x0B => reg_src_human.push_str("r10"),
            0x0C => reg_src_human.push_str("r11"),
            0x0D => reg_src_human.push_str("r12"),
            0x0E => reg_src_human.push_str("PSR"),
            0x0F => reg_src_human.push_str("SP"),

            0x10 => reg_src_human.push_str("LR"),

            _    => reg_src_human.push_str(" "),
        };

        match reg_dest_value {
            0x01 => reg_dest_human.push_str("r0"),
            0x02 => reg_dest_human.push_str("r1"),
            0x03 => reg_dest_human.push_str("r2"),
            0x04 => reg_dest_human.push_str("r3"),

            0x05 => reg_dest_human.push_str("r4"),
            0x06 => reg_dest_human.push_str("r5"),
            0x07 => reg_dest_human.push_str("r6"),
            0x08 => reg_dest_human.push_str("r7"),
            0x09 => reg_dest_human.push_str("r8"),

            0x0A => reg_dest_human.push_str("r9"),
            0x0B => reg_dest_human.push_str("r10"),
            0x0C => reg_dest_human.push_str("r11"),
            0x0D => reg_dest_human.push_str("r12"),
            0x0E => reg_dest_human.push_str("PSR"),
            0x0F => reg_dest_human.push_str("SP"),

            0x10 => reg_dest_human.push_str("LR"),

            _    => reg_dest_human.push_str(" "),
        };

        // everything set, now return the machin code as human readable
        let readable = vec![op_human, reg_src_human, reg_dest_human];

        readable
    }

    pub fn execute(&mut self, instruction_human:Vec<String>) -> Register{
        // now we execute the instruction with the old function
        // we start by reading the registers value and store them locally
        // aka register source and destination. 
        // we should provide the memory map here as input?? because the data are stored there

        // instruction type : [op_human, reg_src_human, reg_dest_human];
        // [OR, r0, r1]
        let op = instruction_human[0].clone();
        let dest = instruction_human[1].clone();
        let src = instruction_human[2].clone();

        
        //read register destination data
        let reg_dest = self.memory_map.register.get(&dest).unwrap_or_else(||{
            println!("error reg_dest");
            process::exit(1);
        });
        let mut reg_src = &Register { name: String::new(), value: 0 };

        // for example a LDI instruction load immediate value into the specified register
        // check if the op is LDI and call the corresponding function
        let addressing_mode: String;

        match op.as_str() {
            "LDI"       =>  {
                addressing_mode = String::from("Data Direct");
                // reg_src is a constant here
            },
            "LDS"       =>  {
                addressing_mode = String::from("Data Direct 2");
                // reg_src is a literal address here
            },
            _           =>  {
                addressing_mode = String::from("Register Direct");
                // read register source data, only valid in Register Direct mode not data
                reg_src = self.memory_map.register.get(&src).unwrap_or_else(||{
                    println!("error reg src");
                    process::exit(1);
                });
            },
        }
       
        let mut rd = Register { name: reg_dest.name.to_string(), value: reg_dest.value };
        let mut reg_result = Register { name: reg_dest.name.clone(), value: reg_dest.value };


        match addressing_mode.as_str() {
            "Register Direct"   =>      {
                reg_result = Instruction::execute_2(op.clone(), &mut rd, reg_src);
            },

            "Data Direct"       =>      {
                // in data direct, there's no source register, instead a literal value
                let literal = u8::from_str_radix((&src).trim_start_matches("0x"), 16).expect("expected an Hex value");
                // set destinarion reg value to the literal
                reg_result = Instruction::execute_load_immediate(reg_dest, literal);
            },
            "Data Direct 2"       =>      {
                // in data direct 2, there's no source register, instead an address to data in memory space
                let address_value = u8::from_str_radix((&src).trim_start_matches("0x"), 16).expect("expected an Hex value");
                reg_result = Instruction::execute_load_from_addr(reg_dest, &address_value, &self.memory_map);
            },

            _                     =>    {
                // just do nothing
            },
        }
        reg_result

    }

    pub fn write_back(&mut self, reg:Register) {
        // take the destination register prev, and update the content in the memory
        self.memory_map.register.insert(reg.name.clone(), reg);
    }



}

fn vect_bin_to_dec(entry: &[u8]) -> u8 {
    let mut value: u8 = 0;
    let binary_base: u8 = 2;
    let mut i= 0;

    for elm in entry.iter().rev(){
        value = {
            value + elm * binary_base.pow(i)
        };
        i = i+1;
    }
    value
}


/*
 * 16 registers r0 to r12 and SP LR PSR
 * SP: Stack pointer, PSR: Process statut Register, LR: Link Register
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
        let PSR= Register::new("PSR", 0x00).unwrap_or_else(|err| {
            println!("error parsing register {}", err);
            process::exit(1);
        });

        let mut reg_hmap: HashMap<String, Register> = HashMap::new();
        let mut stack = Vec::new();
        let psr_temp=Register{value:PSR.value, name:String::from("PSR")};

        // [memory program and datayy]
        // memory is of 16x32 bits or 512 bits or 64 Bytes or 0.064 kilobyte haha
        let mut memory: HashMap<u8, u32> = HashMap::new();

        // program memory : 4x16 bits or 64 bits or 24 Bytes
        memory.insert(0x00, 0x00_00); // at address 0x00, memory contains 0
        memory.insert(0x01, 0x00_00); // at address 0x01, memory contains 0
        memory.insert(0x02, 0x00_00); // at address 0x02, memory contains 0
        memory.insert(0x03, 0x00_00); // at address 0x03, memory contains 0
        
        // data memory : 12x16 bits or 192 bits or 8 Bytes
        memory.insert(0x04, 0x00_00); // at address 0x04, memory contains 0
        memory.insert(0x05, 0x00_00); // at address 0x05, memory contains 0
        memory.insert(0x06, 0x00_00); // at address 0x06, memory contains 0
        memory.insert(0x07, 0x00_00); // at address 0x07, memory contains 0
        memory.insert(0x08, 0x00_00); // at address 0x08, memory contains 0
        memory.insert(0x09, 0x00_00); // at address 0x09, memory contains 0
        
        memory.insert(0x0A, 0x00_00); // at address 0x0A, memory contains 0
        memory.insert(0x0B, 0x00_00); // at address 0x0B, memory contains 0
        memory.insert(0x0C, 0x00_00); // at address 0x0C, memory contains 0
        memory.insert(0x0D, 0x00_00); // at address 0x0D, memory contains 0
        memory.insert(0x0E, 0x00_00); // at address 0x0E, memory contains 0
        memory.insert(0x0F, 0x00_00); // at address 0x0F, memory contains 0

        stack.push(psr_temp);

        //[map the register into the memory]
        reg_hmap.insert(r0.name.clone(), r0);
        reg_hmap.insert(r1.name.clone(), r1);
        reg_hmap.insert(r2.name.clone(), r2);
        reg_hmap.insert(r3.name.clone(), r3);
        reg_hmap.insert(r4.name.clone(), r4);
        reg_hmap.insert(r5.name.clone(), r5);
        reg_hmap.insert(r6.name.clone(), r6);
        reg_hmap.insert(r7.name.clone(), r7);
        reg_hmap.insert(r8.name.clone(), r8);
        reg_hmap.insert(r9.name.clone(), r9);
        reg_hmap.insert(r10.name.clone(), r10);
        reg_hmap.insert(r11.name.clone(), r11);
        reg_hmap.insert(r12.name.clone(), r12);

        reg_hmap.insert(SP.name.clone(), SP);
        reg_hmap.insert(LR.name.clone(), LR);
        reg_hmap.insert(PSR.name.clone(), PSR);

        MemoryMap { register:reg_hmap, stack, memory }
    }
}

impl Register{
    pub fn new(name: &str, value:u8) -> Result<Register, &str>{
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
        let r1_val: u8 = u8::from_str_radix((args[3].clone()).trim_start_matches("0x"), 16).expect("expected an Hex value");

        let r2 = args[4].clone();
        let r2_val: u8 = u8::from_str_radix((args[5].clone()).trim_start_matches("0x"), 16).expect("expected an Hex value");

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

    pub fn execute_load_immediate(register: &Register, literal: u8) -> Register{
        Register { name: register.name.clone(), value: literal}
    }

    pub fn execute_load_from_addr(register: &Register, addr: &u8, memory: &MemoryMap) -> Register{
        // read the memory map and set the input register with it's contents
        let value_at_address = memory.memory.get(addr).unwrap_or_else(||{
            println!("invalid address load");
            process::exit(1);
        });
        let value = *value_at_address;
        // unsafe code here, For now, I have no other choice
        let bytes: [u8; 4] = unsafe {
            transmute(value.to_le()) 
        };
        
        // gotta think about operation requiring more than 1 byte of data
        Register { name: register.name.clone(), value: bytes[0]}
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

    // all tests bellow are passing !
    #[test]
    fn test_load_immediate() {
        /*
         * example
         * LDI  r1, 0x21 ; load literal 0x21 into register r1
        */
        let mut core = Core::new();

        // load the content at address 0x01 in register r0
        let instruction = vec![String::from("LDI"), String::from("r1"), String::from("0x21")];
        
        let return_reg = core.execute(instruction);

        let expected_reg = Register{
            name:String::from("r1"),
            value:33, // expected r1 with value 33 in it
        };
        assert_eq!(return_reg.value, expected_reg.value);
        assert_eq!(return_reg.name, expected_reg.name);

    }
    #[test]
    fn test_load_from_memory() {
        /*
         * example
         * 1 0010  0011  0100  0101  0110  0111  1000
         *  transmute as little endian: we get as result an array of 4 u8 values
         * Little Endian: LSB stored at Lowest address
         * [120, 86, 52, 18]
         * 0111  1000 => 120
         * 0101  0110 => 86
         * 0011  0100 => 52
         * 0001  0010 => 18
        */
        let mut core = Core::new();

        // put 0x12345678 (0001 0010   0011 0100   0101 0110   0111 1000) in memory space at address 0x01
        core.memory_map.memory.insert(0x01, 0x12345678).unwrap_or_else(||{
            println!("Error indexing. Invalid space");
            process::exit(1);
        });

       
        // load the content at address 0x01 in register r0
        let instruction = vec![String::from("LDS"), String::from("r0"), String::from("0x01")];
        
        // should return r0, with value [120, 86, 52, 18] (0111  1000 => 120), 1st byte (u8)
        let return_reg = core.execute(instruction);

        let expected_reg = Register{
            name:String::from("r0"),
            value:120, // expected r0 with value 120 in it
        };
        assert_eq!(return_reg.value, expected_reg.value);
        assert_eq!(return_reg.name, expected_reg.name);

    }
    #[test]
    fn test_core_fetch() {
        // machine code or the program: it has an instruction address and instruction code
        /*
         * Example of application machine code
         * 00000000 <app_name>:
         *     0:       1120           add      r0,     r1
         *     1:       3420           mov      r3,     r1
         *     2:       1120           add      r0,     r1
         *     3:       1120           add      r0,     r1
         *     ...           ...           ...         ...
         *     ff:      1120           add      r0,     r1
         *
        */
        // this section test the fetch and PC incremential system
        let mut core = Core::new();

        // the code is in binary
        core.program.insert(0, vec![/*op code*/0, 0, 0, 1,/*reg dest*/ 0, 0, 0, 1, /*reg src*/ 0, 0, 1, 0, /*unused*/ 0, 0, 0, 0]);
        core.program.insert(1, vec![/*op code*/0, 0, 1, 1,/*reg dest*/ 0, 1, 0, 0, /*reg src*/ 0, 0, 1, 0, /*unused*/ 0, 0, 0, 0]);

        // read this code
        let expected_program_line_0: Vec<u8> = vec![/*op code*/0, 0, 0, 1,/*reg dest*/ 0, 0, 0, 1, /*reg src*/ 0, 0, 1, 0, /*unused*/ 0, 0, 0, 0];
        let fetched_program_line_0 = core.fetch_instruction(); // this will fetch the first instruction, address 0

        assert_eq!(&expected_program_line_0, fetched_program_line_0);
    }
    #[test]
    fn test_core_decode() {
        let core = Core::new();
        // 0001 is the opcode ADD
        // 0001 is r0
        // 0010 is r1
        let instruction_to_decode: Vec<u8> = vec![/*op code*/0, 0, 0, 1,/*reg dest*/ 0, 0, 0, 1, /*reg src*/ 0, 0, 1, 0, /*unused*/ 0, 0, 0, 0];
        
        let instruction_human = core.decode_instruction(&instruction_to_decode);
        let expected_instruction = vec![String::from("ADD"), String::from("r0"), String::from("r1")];

        assert_eq!(instruction_human, expected_instruction);
    }
    #[test]
    fn test_core_write_back() {
        let mut core = Core::new();

        // insert to r0, value 5    
        core.memory_map.register.insert("r0".to_string(), Register { name: String::from("r0"), value: 5 });

        let new_reg = Register{
            name:String::from("r0"),
            value:6,
        };

        // execute write back, should overwrite the r0 register value to new value (15)
        core.write_back(new_reg);

        let expected_reg = core.memory_map.register.get("r0").unwrap_or_else(||{
            println!("error");
            process::exit(1);
        });
        // new_reg moved previously, it's easy to just shadow it
        let new_reg = Register{
            name:String::from("r0"),
            value:6,
        };
        assert_eq!(new_reg.value, expected_reg.value);
        assert_eq!(new_reg.name, expected_reg.name);
    }
    #[test]
    fn test_core_execution() {
        let instruction = vec![String::from("OR"), String::from("r0"), String::from("r1")];
        let mut core = Core::new();

        // update the register set r0=5, r1=9;
        // 0101 OR 1001 => 1101(13) remember, this is Logical OR, NOT ADD
        core.memory_map.register.insert("r0".to_string(), Register { name: String::from("r0"), value: 5 });
        core.memory_map.register.insert("r1".to_string(), Register { name: String::from("r1"), value: 9 });

        // this should execute OR r0, r1   with r0 = 5 = dest, r1 = 9 = src, result saved into r0 should be 13
        let return_reg = core.execute(instruction);

        let expected_reg = Register{
            name:String::from("r0"),
            value:13,
        };

        assert_eq!(return_reg.value, expected_reg.value);
        assert_eq!(return_reg.name, expected_reg.name);
    }



    
    #[test]
    fn or_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x04,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x18,
        };
        let op = String::from("OR");
        let result = Instruction::execute_2(op, &mut r2, &r1);
        assert_eq!(0x1C, result.value);
    }
    #[test] 
    fn and_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x04,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x4C,
        };
        let op = String::from("AND");
        let result = Instruction::execute_2(op, &mut r2, &r1);
        assert_eq!(0x04, result.value);
    }
    #[test]
    #[should_panic]
    fn or_confused_with_add_op_test(){
        let r1 = Register{
            name: String::from("r1"),
            value: 0x01,
        };
        let mut r2 = Register{
            name: String::from("r4"),
            value: 0x4F,
        };
        let op = String::from("OR");
        
        let result = Instruction::execute_2(op, &mut r2, &r1);

        assert_eq!(0x50, result.value);
    }
     
}