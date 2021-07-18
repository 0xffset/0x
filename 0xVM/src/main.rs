#![feature(panic_info_message)]

mod memory;
use std::{collections::HashMap, panic};

use memory::Memory;
mod cpu;
use cpu::{instruction_codes, CPU};

use crate::memory::Byte;

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

    let register_names = vec![
        "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",  // general purpose registers
        "pc",  // program counter
        "acc", // accumulator
        "sr",  // status register
        "sp",  // stack pointer
        "fp",  // frame pointer
    ];

    let register_map: HashMap<&'static str, Byte> = register_names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i as Byte * 4))
        .collect();

    let mut memory = Memory::new(0xFFFF);

    memory.set_byte(0, instruction_codes::PUSH);
    memory.set_word(1, 0x33333335);
    memory.set_byte(5, instruction_codes::PUSH);
    memory.set_word(6, 0x22222225);
    memory.set_byte(10, instruction_codes::PUSH);
    memory.set_word(11, 0x11111115);
    memory.set_byte(15, instruction_codes::MOVR);
    memory.set_word(16, 0x12341235);
    memory.set_byte(20, *register_map.get("r1").unwrap());
    memory.set_byte(21, instruction_codes::MOVR);
    memory.set_word(22, 0x56785675);
    memory.set_byte(26, *register_map.get("r4").unwrap());
    memory.set_byte(27, instruction_codes::PUSH);
    memory.set_word(28, 0x00000000);
    memory.set_byte(32, instruction_codes::CALL);
    memory.set_word(33, 3000);
    memory.set_byte(37, instruction_codes::PUSH);
    memory.set_word(38, 0x44444445);
    memory.set_byte(42, instruction_codes::HALT);

    memory.set_byte(3000, instruction_codes::PUSH);
    memory.set_word(3001, 0x01020105);
    memory.set_byte(3005, instruction_codes::PUSH);
    memory.set_word(3006, 0x03040305);
    memory.set_byte(3010, instruction_codes::PUSH);
    memory.set_word(3011, 0x05060505);
    memory.set_byte(3015, instruction_codes::MOVR);
    memory.set_word(3016, 0x07080705);
    memory.set_byte(3020, *register_map.get("r1").unwrap());
    memory.set_byte(3021, instruction_codes::MOVR);
    memory.set_word(3022, 0x080A0805);
    memory.set_byte(3026, *register_map.get("r8").unwrap());
    memory.set_byte(3027, instruction_codes::RET);


    let mut cpu = CPU::new(memory);
    cpu.run_debug(0xffff-1-84, 0xffff);
}
