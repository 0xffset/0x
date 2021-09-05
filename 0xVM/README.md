# 0xVM

32-Bit VM running on a custom instruction set. 

#### Info

As of now all devices must be added in the main function (might implement a configuration script similar to docker-compose).

### Technical details
 - 8 general purpose registers
 - Simple variable sized screen device with some ansi functionality 
 - Subroutines 

### How to run

`cargo run <program>`<br>
`./vm <program>`
 - `program` must be a valid path or filename to a binary file produced by the assembler


#### <br>Read the datasheet.pdf for more information on registers and instructions.