use std::io::{self, Write};

use crate::memory::{Byte, HalfWord, Word};

use super::Device;

pub struct Screen {
    pub width: HalfWord,
    pub height: HalfWord,
}

#[allow(dead_code)]
impl Screen {
    pub fn new(width: HalfWord, height: HalfWord) -> Screen {
        Screen { width, height }
    }

    fn write(&self, data: &[u8]) {
        match io::stdout().write(data) {
            Ok(_) => {}
            Err(_) => panic!("[VM] Failed to write to stdout"),
        }
    }

    #[inline]
    fn move_to(&self, x: Word, y: Word) {
        self.write(format!("\x1b[{};{}H", y, x).as_bytes());
    }

    /// Code: 0xF4 80 80 80
    #[inline]
    fn clear_screen(&self) {
        self.write(format!("\x1b[2J").as_bytes());
    }

    /// Code: 0xF4 80 80 81
    #[inline]
    fn reset(&self) {
        self.write(format!("\x1b[0m").as_bytes());
    }

    /// Code: 0xF4 80 80 82
    #[inline]
    fn set_bold(&self) {
        self.write(format!("\x1b[1m").as_bytes());
    }

    /// Code: 0xF4 80 80 83
    #[inline]
    fn unset_bold(&self) {
        self.write(format!("\x1b[22m").as_bytes());
    }

    /// Code: 0xF4 80 80 84
    #[inline]
    fn set_underline(&self) {
        self.write(format!("\x1b[4m").as_bytes());
    }

    /// Code: 0xF4 80 80 85
    #[inline]
    fn unset_underline(&self) {
        self.write(format!("\x1b[24m").as_bytes());
    }

    /// Code: 0xF4 80 80 86
    #[inline]
    fn set_blink(&self) {
        self.write(format!("\x1b[5m").as_bytes());
    }

    /// Code: 0xF4 80 80 87
    #[inline]
    fn unset_blink(&self) {
        self.write(format!("\x1b[25m").as_bytes());
    }

    /// Code: 0xF4 80 80 88
    #[inline]
    fn set_italics(&self) {
        self.write(format!("\x1b[3m").as_bytes());
    }

    /// Code: 0xF4 80 80 89
    #[inline]
    fn unset_italics(&self) {
        self.write(format!("\x1b[23m").as_bytes());
    }

    /// Code: 0xF4 80 80 8A
    #[inline]
    fn set_strikethrough(&self) {
        self.write(format!("\x1b[9m").as_bytes());
    }

    /// Code: 0xF4 80 80 8B
    #[inline]
    fn unset_strikethrough(&self) {
        self.write(format!("\x1b[29m").as_bytes());
    }
}

#[allow(dead_code)]
impl Device for Screen {
    #[inline]
    fn get_byte(&self, _: Word) -> Byte {
        0
    }

    #[inline]
    fn get_word(&self, _: Word) -> Word {
        0
    }

    #[inline]
    fn set_byte(&mut self, _: Word, _: Byte) {}

    fn set_word(&mut self, addr: Word, word: Word) {
        match word {
            0xF4_80_80_80 => self.clear_screen(),
            0xF4_80_80_81 => self.reset(),
            0xF4_80_80_82 => self.set_bold(),
            0xF4_80_80_83 => self.unset_bold(),
            0xF4_80_80_84 => self.set_underline(),
            0xF4_80_80_85 => self.unset_underline(),
            0xF4_80_80_86 => self.set_blink(),
            0xF4_80_80_87 => self.unset_blink(),
            0xF4_80_80_88 => self.set_italics(),
            0xF4_80_80_89 => self.unset_italics(),
            0xF4_80_80_8A => self.set_strikethrough(),
            0xF4_80_80_8B => self.unset_strikethrough(),
            _ => {
                let x = ((addr & 0x0000FFFF) % self.width as Word) + 1;
                let y = ((addr & 0x0000FFFF) / self.height as Word) + 1;
                self.move_to(x * 2, y);

                self.write(&word.to_le_bytes());
            }
        }
    }
}
