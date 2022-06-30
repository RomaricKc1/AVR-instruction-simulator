# AVR-instruction-simulator
Starting with Rust programming, simple UAL

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
