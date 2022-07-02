# Whole Processor simulator, with ASM program file to execute.
### On-going development(it's my First Rust program so I --for sure-- have made mistakes.

### TODOS
- Read a machine code as shown in the test section

```assambly

    00000000 <app_name>:
        0:       1120           add      r0,     r1
        1:       3420           mov      r3,     r1
        2:       1120           add      r0,     r1
        3:       1120           add      r0,     r1
        ...           ...           ...         ...
        
        ff:      1120           add      r0,     r1
```
- Remove old run mode
- Add instruction, to save data in memory
- Write an example program in machine code
- Add a dump mode to print all the memory and register contents


# instruction-simulator
Starting with Rust programming, simple UAL

## Processor architecture
- 16 register from r0 to r15
- 16 memory block divided into program and data each block has 32 bits total 512 bit or 64 bytes

## Register (16)
| Register | addr | Register | addr |
| ------ | ------ | ------ | ------ |
| r0 | 0x01 | r8 | 0x09 |
| r1 | 0x02 | r9 | 0x0A |
| r2 | 0x03 | r10 | 0x0B |
| r3 | 0x04 | r11 | 0x0C |
| r4 | 0x05 | r12 | 0x0D |
| r5 | 0x06 | PSR | 0x0E |
| r6 | 0x07 | SP | 0x0F |
| r7 | 0x08 | LR | 0x10 |

(to add more)

## Instruction Set :: Control (0, 5, 6)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| nop | -- | 0000 | 0000 0000 0000 | nop operation, cpu does onothing |
| push | sp? | 0101 | 0000 0000 0000 | push to the stack pointer |
| pop | sp? | 0110 | 0000 0000 0000 | pop the stack pointer |


## Instruction Set :: Logic (1-4)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| add | rd, rs | 0001 | rd rd rd rd   rs rs rs rs 0000 | add rs to rd and return rd |
| sub | rd ,rs | 0010 | rd rd rd rd   rs rs rs rs 0000 | substract rs to rd and return rd |
| or | rd, rs | 0011 | rd rd rd rd   rs rs rs rs 0000 | logical or, rs to rd and return rd |
| and | rd, rs | 0100 | rd rd rd rd   rs rs rs rs 0000 | logical and, rs to rd and return rd |

## Instruction Set :: Changle flow (7)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| jmp | addr | 0111 | adr adr adr adr adr adr adr adr 0000| jump to address addr |

## Instruction Set :: Data transfer (8-10)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| ldi | rd, K | 1000 | rd rd rd rd   K K K K 0000 | Load Immediate the literal K to register rd |
| lds | rd, k | 1001 | rd rd rd rd   k k k k 0000 | Load Direct from Data Space at addresse k to register rd |
| mov | rd, rs | 1010 | rd rd rd rd   rs rs rs rs 0000 | move rs to rd and return rd |

## Memory Map (512 bits)
| Addresse (1 byte) | Value (4 bytes) | Addresse (1 byte) | Value (4 bytes)  |
| ------ | ------ | ------ | ------ |
| 0x00 | 0x0000_0000 | 0x08 | 0x0000_0000 |
| 0x01 | 0x0000_0000 | 0x09 | 0x0000_0000 |
| 0x02 | 0x0000_0000 | 0x0A | 0x0000_0000 |
| 0x03 | 0x0000_0000 | 0x0B | 0x0000_0000 |
| 0x04 | 0x0000_0000 | 0x0C | 0x0000_0000 |
| 0x05 | 0x0000_0000 | 0x0D | 0x0000_0000 |
| 0x06 | 0x0000_0000 | 0x0E | 0x0000_0000 |
| 0x07 | 0x0000_0000 | 0x0F | 0x0000_0000 |








## Example of Instruction execution
r2 is register 0011, r3 is register 0100, remaining unused
```
machine code : 0011 0011 0100 0000

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
