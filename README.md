# Whole Processor simulator, with ASM program file to execute.
### On-going developement

# instruction-simulator
Starting with Rust programming, simple UAL

## Processor architecture
- 16 register from r0 to r15
- 16 memory block divided into program and data each block has 32 bits total 512 bit or 64 bytes

## Register
| Register | addr |
| ------ | ------ |
| r0 | 0x01 |
| r1 | 0x02 |
| r2 | 0x03 |
| r3 | 0x04 |
| r4 | 0x05 |
| r5 | 0x06 |
| r6 | 0x07 |
| r7 | 0x08 |
| r8 | 0x09 |
| r9 | 0x0A |
| r10 | 0x0B |
| r11 | 0x0C |
| r12 | 0x0D |
| PC | 0x0E |
| SP | 0x0F |
| LR | 0x10 |


## Instruction Set (to add more)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| nop | -- | 0000 | 0000 0000 0000 | nop operation, cpu does onothing |
| add | rd, rs | 0001 | rd rd rd rd   rs rs rs rs 0000 | add rs to rd and return rd |
| sub | rd ,rs | 0010 | rd rd rd rd   rs rs rs rs 0000 | substract rs to rd and return rd |
| mov | rd, rs | 0011 | rd rd rd rd   rs rs rs rs 0000 | move rs to rd and return rd |
| jmp | addr | 0100 | adr adr adr adr adr adr adr adr 0000 0000| jump to address addr |
| push | sp? | 0101 | 0000 0000 0000 | push the stack pointer |
| pop | sp? | 0110 | 0000 0000 0000 | op the stack pointer |
| jmp | -- | 0111 | 0000 0000 0000 | -- |
| jmp | -- | 1000 | 0000 0000 0000 | -- |

## Example of Instruction execution
r2 is register 0011, r3 is register 0100, remaining unused
```
machine code : 0011 0011 0100 0000 0000

same as 
ASM          : MOV r2, r3 ;move content of r3 into r2
```





























### Old, on revision

### RUN
```bash
cargo run Register_Direct r1 0x03 r2 0x04 OR
```
### std output
```Rust
AVR-8 I simulatior  
################################  
Execution mode: Register Direct  
Intruction execution  
Op Code 		     reg src		         reg dest  
--------------------------------------------------------------------------------------
OR 		           r1[value: 3] 		   r2[value: 4, new value: 7] 	  	
```
