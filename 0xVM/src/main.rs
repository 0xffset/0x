#![feature(panic_info_message)]

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

    
    let memory = Memory::new(0xFF);
    let mut cpu = CPU::new(memory);

    cpu.memory.set_byte(0, instruction_codes::MOVR);
    cpu.memory.set_word(1, 0x12345678);
    cpu.memory.set_byte(5, *cpu.register_map.get("r1").unwrap());

    cpu.memory.set_byte(6, instruction_codes::PUSH);
    cpu.memory.set_word(7, 0x90ABCDEF);
    cpu.memory.set_byte(11, instruction_codes::PUSHR);
    cpu.memory.set_byte(12, *cpu.register_map.get("r1").unwrap());

    cpu.memory.set_byte(13, instruction_codes::POP);
    cpu.memory.set_byte(14, *cpu.register_map.get("r3").unwrap());

    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.view_memory_at(0xF0, 0xFF)

    /*
    cpu.memory.set_byte(0, instruction_codes::MOVR);
    cpu.memory.set_word(1, 0x1234);
    cpu.memory.set_byte(5, *cpu.register_map.get("r1").unwrap());

    cpu.memory.set_byte(6, instruction_codes::MOVR);
    cpu.memory.set_word(7, 0xABCD);
    cpu.memory
        .set_byte(11, *cpu.register_map.get("r2").unwrap());

    cpu.memory.set_byte(12, instruction_codes::ADDR);
    cpu.memory
        .set_byte(13, *cpu.register_map.get("r1").unwrap());
    cpu.memory.set_byte(14, *cpu.register_map.get("r2").unwrap());

    cpu.memory.set_byte(15, instruction_codes::MOVRM);
    cpu.memory
        .set_byte(16, *cpu.register_map.get("acc").unwrap());
    cpu.memory.set_word(17, 0x3A);
	cpu.memory.set_byte(21, instruction_codes::HALT);

    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
    cpu.step();
    cpu.debug();
	cpu.step();
	cpu.debug();
    cpu.view_memory_at(0, 0x4C);
    */

}
