use std::convert::TryInto;

use crate::device::Device;

pub type Word = u32;
pub type Byte = u8;

#[derive(Clone, PartialEq)]
pub struct Memory {
    data: Vec<Byte>,
}

impl Memory {
    pub fn new(size_in_bytes: Word) -> Memory {
        Memory {
            data: vec![0; size_in_bytes as usize],
        }
    }

    pub fn get_size(&self) -> Word {
        self.data.len() as Word
    }

    /// Masks byte with bitwise-and at address
    pub fn and_set_byte(&mut self, address: Word, mask: Byte) {
        if self.data.len() < address as usize {
            panic!("[MEMORY] and_set_byte: No such address '0x{:08X}'", address);
        }

        self.data[address as usize] &= mask;
    }

    /// Masks word with bitwise-or at address
    pub fn or_set_byte(&mut self, address: Word, mask: Byte) {
        if self.data.len() < address as usize {
            panic!("[MEMORY] or_set_byte: No such address '0x{:08X}'", address);
        }

        self.data[address as usize] |= mask;
    }
}

impl Device for Memory {
    fn get_byte(&self, address: Word) -> Byte {
        *self
            .data
            .get(address as usize)
            .expect(format!("[MEMORY] get_byte: No such address '0x{:08X}'", address).as_str())
    }

    fn get_word(&self, address: Word) -> Word {
        let data = self
            .data
            .get(address as usize..address as usize + 4)
            .expect(format!("[MEMORY] get_word: No such address '0x{:08X}'", address).as_str())
            .try_into()
            .expect("[MEMORY] get_word: Oddly sized Word");

        Word::from_le_bytes(data)
    }

    fn set_byte(&mut self, address: Word, byte: Byte) {
        if self.data.len() < address as usize {
            panic!("[MEMORY] set_byte: No such address '0x{:08X}'", address);
        }

        self.data[address as usize] = byte;
    }

    fn set_word(&mut self, address: Word, word: Word) {
        if self.data.len() < address as usize + 3 {
            panic!("[MEMORY] set_word: No such address '0x{:08X}'", address);
        }

        for (i, byte) in word.to_le_bytes().iter().enumerate() {
            self.data[address as usize + i] = *byte;
        }
    }
}
