#![feature(panic_info_message)]

use macros::{init_registers, reg};

init_registers![
    "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", // general purpose registers
    "pc", // program counter
    "acc", // accumulator
    "sr", // status register
    "sp", // stack pointer
    "fp", // frame pointer
];

mod memory;
use std::panic;

use memory::Memory;
mod cpu;
use cpu::{instruction_codes, CPU};

fn main() {
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.message() {
            println!("0xVM panicked at\n{}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("0xVM panicked at\n{}", s);
        } else {
            println!("0xVM panicked!");
        }
    }));

    let mut memory = Memory::new(0xFFFF);

    memory.set_byte(0, instruction_codes::PUSH);
    memory.set_word(1, 0x33333335);
    memory.set_byte(5, instruction_codes::PUSH);
    memory.set_word(6, 0x22222225);
    memory.set_byte(10, instruction_codes::PUSH);
    memory.set_word(11, 0x11111115);
    memory.set_byte(15, instruction_codes::MOVR);
    memory.set_word(16, 0x12341235);
    memory.set_word(20, reg!("r1"));
    memory.set_byte(24, instruction_codes::MOVR);
    memory.set_word(25, 0x56785675);
    memory.set_word(29, reg!("r4"));
    memory.set_byte(33, instruction_codes::PUSH);
    memory.set_word(34, 0x00000000);
    memory.set_byte(38, instruction_codes::CALL);
    memory.set_word(39, 3000);
    memory.set_byte(43, instruction_codes::PUSH);
    memory.set_word(44, 0x44444445);
    memory.set_byte(48, instruction_codes::HALT);

    memory.set_byte(3000, instruction_codes::PUSH);
    memory.set_word(3001, 0x01020105);
    memory.set_byte(3005, instruction_codes::PUSH);
    memory.set_word(3006, 0x03040305);
    memory.set_byte(3010, instruction_codes::PUSH);
    memory.set_word(3011, 0x05060505);
    memory.set_byte(3015, instruction_codes::MOVR);
    memory.set_word(3016, 0x07080705);
    memory.set_word(3020, reg!("r1"));
    memory.set_byte(3024, instruction_codes::MOVR);
    memory.set_word(3025, 0x080A0805);
    memory.set_word(3029, reg!("r8"));
    memory.set_byte(3033, instruction_codes::RET);


    let mut cpu = CPU::new(memory);
    cpu.run_debug(0xffff-1-84, 0xffff);
}
