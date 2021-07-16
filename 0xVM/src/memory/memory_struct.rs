use std::convert::TryInto;

pub type Word = u32;
pub type Byte = u8;

pub enum Flags {
    Z,
    O
}

pub struct Memory {
    data: Vec<Byte>,
}

impl Memory {
    pub fn new(size_in_bytes: usize) -> Memory {
        Memory {
            data: vec![0; size_in_bytes],
        }
    }

    pub fn get_size(&self) -> Word {
        self.data.len() as Word
    }

    pub fn get_byte(&self, address: Word) -> Byte {
        *self
            .data
            .get(address as usize)
            .expect(format!("[MEMORY] get_byte: No such address '0x{:08X}'", address).as_str())
    }

    pub fn get_word(&self, address: Word) -> Word {
        let data = self
            .data
            .get(address as usize..address as usize + 4)
            .expect(format!("[MEMORY] get_word: No such address '0x{:08X}'", address).as_str())
            .try_into().expect("[MEMORY] get_word: Oddly sized Word");

        Word::from_le_bytes(data)
    }

	pub fn set_byte(&mut self, address: Word, byte: Byte) {
		if self.data.len() < address as usize {
			panic!("[MEMORY] set_byte: No such address '0x{:08X}'", address);
		}

		self.data[address as usize] = byte;
	}

    pub fn set_word(&mut self, address: Word, word: Word) {
        if self.data.len() < address as usize + 3 {
            panic!("[MEMORY] set_word: No such address '0x{:08X}'", address);
        }

        for (i, byte) in word.to_le_bytes().iter().enumerate() {
            self.data[address as usize + i] = *byte;
        }
    }

    pub fn set_flag(&mut self, address: Byte, flag: Flags, value: bool) {
        match flag {
            Flags::Z => {
                if value {
                    self.data[address as usize] |= 0x01;
                } else {
                    self.data[address as usize] &= 0xFE;
                }
            }
            Flags::O => {
                if value {
                    self.data[address as usize] |= 0x02;
                } else {
                    self.data[address as usize] &= 0xFD;
                }
            }
        }
    }
}
