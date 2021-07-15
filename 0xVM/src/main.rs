mod memory;
use memory::Memory;

mod cpu;
use cpu::{CPU, instruction_codes};


fn main() {
	let memory = Memory::new(256);
	let mut cpu = CPU::new(memory);
	cpu.memory.set_byte(0, instruction_codes::MOVI_WORD_REG);
	cpu.memory.set_word(1, 0x1234);
	cpu.memory.set_byte(5, *cpu.register_map.get("r1").unwrap());
	
	cpu.memory.set_byte(6, instruction_codes::MOVI_WORD_REG);
	cpu.memory.set_word(7, 0xABCD);
	cpu.memory.set_byte(11, *cpu.register_map.get("r2").unwrap());
	
	cpu.memory.set_byte(12, instruction_codes::ADD_REG_REG);
	cpu.memory.set_byte(13, *cpu.register_map.get("r1").unwrap());
	cpu.memory.set_byte(14, *cpu.register_map.get("r2").unwrap()); 

	cpu.debug();
	cpu.step();
	cpu.debug();
	cpu.step();
	cpu.debug();
	cpu.step();
	cpu.debug();
}