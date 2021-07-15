use std::{collections::HashMap};

use crate::memory::{Byte, Memory, Word};
use super::instructions::instruction_codes;

macro_rules! String {
    ($s:expr) => {
        String::from($s)
    };
}

pub struct CPU {
    pub memory: Memory,
    register_names: Vec<String>,
    registers: Memory,
    pub register_map: HashMap<String, Byte>,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        let register_names = vec![
            String!("pc"),
            String!("acc"),
            String!("r1"),
            String!("r2"),
            String!("r3"),
            String!("r4"),
            String!("r5"),
            String!("r6"),
            String!("r7"),
            String!("r8"),
        ];
        let register_names_len = register_names.len();

        let register_map: HashMap<String, Byte> = register_names
            .iter()
            .enumerate()
            .map(|(i, n)| (n.clone(), i as Byte * 4))
            .collect();

        CPU {
            memory,
            register_names,
            registers: Memory::new(register_names_len * 4),
            register_map,
        }
    }

    fn get_register_by_name(&self, name: String) -> Word {
        self.registers.get_word(
            *self
                .register_map
                .get(&name)
                .expect(format!("[CPU] get_register: No such register '{}'", name).as_str()) as Word,
        )
    }

	fn get_register_by_address(&self, address: Byte) -> Word {
		self.registers.get_word(address as Word)
	}

    fn set_register_by_name(&mut self, name: String, value: Word) {
        self.registers.set_word(
            *self
                .register_map
                .get(&name)
                .expect(format!("[CPU] set_register: No such register '{}'", name).as_str()) as Word,
            value,
        );
    }

	fn set_register_by_address(&mut self, address: Byte, value: Word) {
		self.registers.set_word(address as Word, value);
	}

	fn fetch_byte(&mut self) -> Byte {
		let next_instruction_address = self.get_register_by_name(String!("pc"));
		self.set_register_by_name(String!("pc"), next_instruction_address + 1);
		
		self.memory.get_byte(next_instruction_address)
	}

	fn fetch_word(&mut self) -> Word {
		let next_instruction_address = self.get_register_by_name(String!("pc"));
		self.set_register_by_name(String!("pc"), next_instruction_address + 4);

		self.memory.get_word(next_instruction_address)
	}

	fn execute(&mut self, instruction: Byte) {
		match instruction {
			// movi 0x1234, r1
			instruction_codes::MOVI_WORD_REG => {
				let value = self.fetch_word();
				let register_address = self.fetch_byte();
				self.set_register_by_address(register_address, value);
			},
			// add r1, r2
			instruction_codes::ADD_REG_REG => {
				let register1_address = self.fetch_byte();
				let register2_address = self.fetch_byte();

				let reigster1_value = self.get_register_by_address(register1_address);
				let register2_value = self.get_register_by_address(register2_address);

				self.set_register_by_name(String!("acc"), reigster1_value + register2_value);
			},
			_ => {}
		}
	}

    pub fn debug(&self) {
        for name in self.register_names.clone() {
            println!("{:>4}: 0x{:08X}", name, self.get_register_by_name(name.clone()));
        }
        println!("");
    }

	pub fn step(&mut self) {
		let instruction = self.fetch_byte();
		self.execute(instruction);
	}
}
