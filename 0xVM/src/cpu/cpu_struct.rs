use std::collections::HashMap;

use crate::memory::{Byte, Flags, Memory, Word};

use super::instruction_codes;

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
    halt_signal: bool,
}

impl CPU {
    pub fn new(memory: Memory) -> CPU {
        let register_names = vec![
            String!("pc"),
            String!("acc"),
            String!("sr"),
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
            halt_signal: false,
        }
    }

    fn update_status_register(&mut self, pre: Word, post: Word) {
        self.registers.set_flag(
            *self.register_map.get("sr").unwrap(),
            Flags::Z,
            post == 0,
        );

        self.registers.set_flag(
            *self.register_map.get("sr").unwrap(),
            Flags::O,
            pre > post,
        );
    }

    /// Gets the value of the register with the given name.
    fn get_register_by_name(&self, name: String) -> Word {
        self.registers.get_word(
            *self
                .register_map
                .get(&name)
                .expect(format!("[CPU] get_register: No such register '{}'", name).as_str())
                as Word,
        )
    }

    /// Gets the value of the register with the given address.
    fn get_register_by_address(&self, address: Byte) -> Word {
        self.registers.get_word(address as Word)
    }

    /// Sets the value of the register with the given name.
    fn set_register_by_name(&mut self, name: String, value: Word) {
        self.registers.set_word(
            *self
                .register_map
                .get(&name)
                .expect(format!("[CPU] set_register: No such register '{}'", name).as_str())
                as Word,
            value,
        );
    }

    /// Sets the value of the register with the given address.
    fn set_register_by_address(&mut self, address: Byte, value: Word) {
        self.registers.set_word(address as Word, value);
    }

    /// Fetches the next byte from memory and increments the program counter.
    fn fetch_byte(&mut self) -> Byte {
        let next_instruction_address = self.get_register_by_name(String!("pc"));
        self.set_register_by_name(String!("pc"), next_instruction_address + 1);

        self.memory.get_byte(next_instruction_address)
    }

    /// Fetches the next word from memory and increments the program counter.
    fn fetch_word(&mut self) -> Word {
        let next_instruction_address = self.get_register_by_name(String!("pc"));
        self.set_register_by_name(String!("pc"), next_instruction_address + 4);

        self.memory.get_word(next_instruction_address)
    }

    fn execute(&mut self, instruction: Byte) {
        match instruction {
            instruction_codes::HALT => {
                self.halt_signal = true;
            }
            instruction_codes::NOP => {}
            // movri 0x1234, r1 -> Move 0x1234 into register r1
            instruction_codes::MOV_WORD_REG => {
                let value = self.fetch_word();
                let register_address = self.fetch_byte();
                self.set_register_by_address(register_address, value);
            }
            // movmi 0x1234, 0x0000 00AF -> Move 0x1234 into memory at 0x0000 00AF
            instruction_codes::MOV_WORD_MEM => {
                let value = self.fetch_word();
                let memory_address = self.fetch_word();
                self.memory.set_word(memory_address, value);
            }
            // movr r1, r2 -> Move r1 to r2
            instruction_codes::MOV_REG_REG => {
                let register1_address = self.fetch_byte();
                let register2_address = self.fetch_byte();
                self.set_register_by_address(
                    register2_address,
                    self.get_register_by_address(register1_address),
                );
            }
            // movrm r1, 0x0000 00AF -> Move r1 to 0x0000 00AF
            instruction_codes::MOV_REG_MEM => {
                let register_address = self.fetch_byte();
                let memory_address = self.fetch_word();
                self.memory.set_word(
                    memory_address,
                    self.get_register_by_address(register_address),
                );
            }
            // movmr 0x0000 00AF, r1 -> Move 0x0000 00AF to r1
            instruction_codes::MOV_MEM_REG => {
                let memory_address = self.fetch_word();
                let register_address = self.fetch_byte();
                self.set_register_by_address(
                    register_address,
                    self.memory.get_word(memory_address),
                );
            }
            // addr r1, r2 -> Add r1 to r2 and store the result in acc
            instruction_codes::ADD_REG_REG => {
                let register1_address = self.fetch_byte();
                let register2_address = self.fetch_byte();

                let register1_value = self.get_register_by_address(register1_address);
                let register2_value = self.get_register_by_address(register2_address);

                let acc = register1_value.wrapping_add(register2_value);

                self.set_register_by_name(String!("acc"), acc);

                self.update_status_register(register1_value, acc);
            }
            _ => {
                panic!("[CPU] No such instruction: '0x{:02X}'", instruction);
            }
        }
    }

    pub fn debug(&self) {
        for name in self.register_names.clone() {
            println!(
                "{:<4}: 0x{:08X}",
                name,
                self.get_register_by_name(name.clone())
            );
        }
        println!();
    }

    pub fn view_memory_at(&self, address: Word, n: Word) {
        let mut mem_snapshot: Vec<Byte> = Vec::new();
        let max_address = if address + n < self.memory.get_size() {
            address + n
        } else {
            self.memory.get_size()
        };

        for i in address..max_address {
            mem_snapshot.push(self.memory.get_byte(i));
        }

        for (offset, byte) in mem_snapshot.iter().enumerate() {
            println!("0x{:08X}: {:02X}", address as usize + offset, byte);
        }
        println!();
    }

    /// Progresses the program
    pub fn step(&mut self) {
        let instruction = self.fetch_byte();
        self.execute(instruction);
    }
}
