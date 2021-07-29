use std::convert::TryInto;

use crate::device::Device;

pub type Word = u32;
pub type HalfWord = u16;
pub type Byte = u8;

#[derive(Clone, PartialEq)]
pub struct Memory {
    data: Vec<Byte>,
}

#[allow(dead_code)]
impl Memory {
    pub fn new(size_in_bytes: Word) -> Self {
        Memory {
            data: vec![0; size_in_bytes as usize],
        }
    }

    pub fn from(mut data: Vec<Byte>, total_size: Word) -> Self {
        let buffer = if total_size > data.len() as Word {
            total_size - data.len() as Word
        } else {
            0
        };
        
        for _ in 0..buffer {
            data.push(0);
        }

        Memory { data }
    }

    pub fn get_size(&self) -> Word {
        self.data.len() as Word
    }

    /// Masks byte with bitwise-and at addr
    pub fn and_set_byte(&mut self, addr: Word, mask: Byte) {
        if self.data.len() < addr as usize {
            panic!("[MEMORY] and_set_byte: No such addr '0x{:08X}'", addr);
        }

        self.data[addr as usize] &= mask;
    }

    /// Masks word with bitwise-or at addr
    pub fn or_set_byte(&mut self, addr: Word, mask: Byte) {
        if self.data.len() < addr as usize {
            panic!("[MEMORY] or_set_byte: No such addr '0x{:08X}'", addr);
        }

        self.data[addr as usize] |= mask;
    }
}

#[allow(dead_code)]
impl Device for Memory {
    fn get_byte(&self, addr: Word) -> Byte {
        *self
            .data
            .get(addr as usize)
            .expect(format!("[MEMORY] get_byte: No such addr '0x{:08X}'", addr).as_str())
    }

    fn get_word(&self, addr: Word) -> Word {
        let data = self
            .data
            .get(addr as usize..addr as usize + 4)
            .expect(format!("[MEMORY] get_word: No such addr '0x{:08X}'", addr).as_str())
            .try_into()
            .expect("[MEMORY] get_word: Oddly sized Word");

        Word::from_le_bytes(data)
    }

    fn set_byte(&mut self, addr: Word, byte: Byte) {
        if self.data.len() < addr as usize {
            panic!("[MEMORY] set_byte: No such addr '0x{:08X}'", addr);
        }

        self.data[addr as usize] = byte;
    }

    fn set_word(&mut self, addr: Word, word: Word) {
        if self.data.len() < addr as usize + 3 {
            panic!("[MEMORY] set_word: No such addr '0x{:08X}'", addr);
        }

        for (i, byte) in word.to_le_bytes().iter().enumerate() {
            self.data[addr as usize + i] = *byte;
        }
    }
}
