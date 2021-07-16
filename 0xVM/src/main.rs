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

    let memory = Memory::new(128);
    let mut cpu = CPU::new(memory);
    cpu.memory.set_byte(0, instruction_codes::MOV_WORD_REG);
    cpu.memory.set_word(1, 0x1234);
    cpu.memory.set_byte(5, *cpu.register_map.get("r1").unwrap());

    cpu.memory.set_byte(6, instruction_codes::MOV_WORD_REG);
    cpu.memory.set_word(7, 0xABCD);
    cpu.memory
        .set_byte(11, *cpu.register_map.get("r2").unwrap());

    cpu.memory.set_byte(12, instruction_codes::ADD_REG_REG);
    cpu.memory
        .set_byte(13, *cpu.register_map.get("r1").unwrap());
    cpu.memory.set_byte(14, *cpu.register_map.get("r2").unwrap());

    cpu.memory.set_byte(15, instruction_codes::MOV_REG_MEM);
    cpu.memory
        .set_byte(16, *cpu.register_map.get("acc").unwrap());
    cpu.memory.set_word(17, 0x4A);
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
}
