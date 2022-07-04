# AVR-like Instruction set simulator
Starting with `Rust` programming, simple UAL.

Concepts learned:
```
ownership
structure
patern matching
data collection
error handling
testing
closures
iterators
conditions ...
```

### On-going development(it's my First Rust program so I `for sure` have made mistakes.
### Can we run DOOM on it?
```rust
fn main() {
    println!("Sure, DOOM runs on everything :)");
}
```
### TODOS
- Add conditional jump ( branche if not equal, jump, ...)
- Store multiple u8 data in the u32 memory space, not a single u8
- Add logical and arithmetical operation
- Use the last 1/2 byte of the instruction in order to manipulate values > 0xff
- Better parse system to read instruction like this :
```assambly

    00000000 <app_name>:
        0:       1120           add      r0,     r1
        1:       3420           mov      r3,     r1
        2:       1120           add      r0,     r1
        3:       1120           add      r0,     r1
        ...           ...           ...         ...
        
        ff:      1120           add      r0,     r1
```

### RUN
```bash
cargo run application/prog.S 
```
### prog.S (note `\n` at the end of each line, including _start and _end lines)
```assambly
_start
0x8190; ldi r0, 0x09
0xBB10; st  0x0B, r0
0x92B0; lds r1, 0x0B
_end

```

### std output
```Rust
AVR-like Instruction Set simulator
Memory Map 
Address (u8) 		 Value (u32) 
9 	 => 		 0
15 	 => 		 0
8 	 => 		 0
6 	 => 		 37552
1 	 => 		 0
3 	 => 		 0
11 	 => 		 9
12 	 => 		 0
13 	 => 		 0
2 	 => 		 0
7 	 => 		 0
0 	 => 		 0
10 	 => 		 0
14 	 => 		 0
5 	 => 		 47888
4 	 => 		 33168
Registers 
Register name) 		 Value (u8) 
r9 	 => 		 0
r1 	 => 		 9
LR 	 => 		 0
r11 	 => 		 0
PSR 	 => 		 0
r6 	 => 		 0
r10 	 => 		 0
r7 	 => 		 0
r2 	 => 		 0
r12 	 => 		 0
r3 	 => 		 0
r0 	 => 		 9
r5 	 => 		 0
r8 	 => 		 0
SP 	 => 		 0
r4 	 => 		 0
  	
```

## Memory dump after loading a program 1
### test program
```assambly
_start
     0  0x8190; ldi r0, 0x09
     1  0xBB10; st  0x0B, r0
     2  0x92B0; lds r1, 0x0B
_end
```
### Memory Map and register Map (register r0 and r1 have their new content
```rust
Memory Map 
Address (u8) 		 Value (u32) 
2 	 => 		 0
0 	 => 		 0
1 	 => 		 0
13 	 => 		 0
7 	 => 		 0
5 	 => 		 47888
12 	 => 		 0
3 	 => 		 0
4 	 => 		 33168
11 	 => 		 9
14 	 => 		 0
8 	 => 		 0
9 	 => 		 0
6 	 => 		 37552
10 	 => 		 0
15 	 => 		 0
Registers 
Register name) 		 Value (u8) 
SP 	 => 		 0
r4 	 => 		 0
r10 	 => 		 0
r11 	 => 		 0
r9 	 => 		 0
r3 	 => 		 0
r1 	 => 		 9
r2 	 => 		 0
r7 	 => 		 0
LR 	 => 		 0
PSR 	 => 		 0
r5 	 => 		 0
r6 	 => 		 0
r12 	 => 		 0
r8 	 => 		 0
r0 	 => 		 9
```

## Memory dump after loading a program 2
### test program
```assambly
_start
     0   0x1120; add r0, r1
     1   0xA510; mov r4, r0
     2   0x8150; ldi r0, 5
     3   0x9210; lds r1, 0x01
     4   0x4520; and r4, r1
     5   0x3510; or r4, r0
_end
```
### Memory Map
Program start at address 0x04 in memory, so 0x04, 0x05, 0x06, 0x07, 0x08, 0x09 are modified with their respective machine code. 
```rust
Memory Map 
Address (u8) 		        Value (u32) 
9 	        => 		 13584
10 	        => 		 0
2 	        => 		 0
13 	        => 		 0
6 	        => 		 33104
14 	        => 		 0
5 	        => 		 42256
3 	        => 		 0
15 	        => 		 0
1 	        => 		 0
0 	        => 		 0
12 	        => 		 0
4 	        => 		 4384
11 	        => 		 0
7 	        => 		 37392
8 	        => 		 17696

```

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

## Instruction Set :: Change flow (7)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| jmp | addr | 0111 | adr adr adr adr adr adr adr adr 0000| jump to address addr |

## Instruction Set :: Data transfer (8-10)
| op | operands | binary | operands | comment |
| ------ | ------ | ------ | ------ | ------ |
| ldi | rd, K | 1000 | rd rd rd rd   K K K K 0000 | Load Immediate the literal K to register rd |
| lds | rd, k | 1001 | rd rd rd rd   k k k k 0000 | Load Direct from Data Space at addresse k to register rd |
| mov | rd, rs | 1010 | rd rd rd rd   rs rs rs rs 0000 | move rs to rd and return rd |
| st | k, rs | 1011 | k k k k   rs rs rs rs 0000 | Sore direct the content of rs into the memory address k |


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
