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

    fn move_to(&self, x: Word, y: Word) {
        self.write(format!("\x1b[{};{}H", y, x).as_bytes());
    }

    /// Code: 0xFF FF
    fn clear_screen(&self) {
        self.write(format!("\x1b[2J").as_bytes());
    }

    /// Code: 0xFF FE
    fn reset(&self) {
        self.write(format!("\x1b[0m").as_bytes());
    }

    /// Code: 0x00 01
    fn set_bold(&self) {
        self.write(format!("\x1b[1m").as_bytes());
    }

    /// Code: 0x00 02
    fn unset_bold(&self) {
        self.write(format!("\x1b[22m").as_bytes());
    }

    /// Code: 0x00 03
    fn set_underline(&self) {
        self.write(format!("\x1b[4m").as_bytes());
    }

    /// Code: 0x00 04
    fn unset_underline(&self) {
        self.write(format!("\x1b[24m").as_bytes());
    }

    /// Code: 0x00 05
    fn set_blink(&self) {
        self.write(format!("\x1b[5m").as_bytes());
    }

    /// Code: 0x00 06
    fn unset_blink(&self) {
        self.write(format!("\x1b[25m").as_bytes());
    }

    /// Code: 0x00 07
    fn set_italics(&self) {
        self.write(format!("\x1b[3m").as_bytes());
    }

    /// Code: 0x00 08
    fn unset_italics(&self) {
        self.write(format!("\x1b[23m").as_bytes());
    }

    /// Code: 0x00 09
    fn set_strikethrough(&self) {
        self.write(format!("\x1b[9m").as_bytes());
    }

    /// Code: 0x00 0A
    fn unset_strikethrough(&self) {
        self.write(format!("\x1b[29m").as_bytes());
    }
}

#[allow(dead_code)]
impl Device for Screen {
    fn get_byte(&self, _: Word) -> Byte {
        0
    }

    fn get_word(&self, _: Word) -> Word {
        0
    }

    fn set_byte(&mut self, _: Word, _: Byte) {}

    fn set_word(&mut self, address: Word, word: Word) {
        let command = (word & 0xFFFF0000) >> 16;

        match command {
            0xFFFF => self.clear_screen(),
            0xFFFE => self.reset(),
            0x1 => self.set_bold(),
            0x2 => self.unset_bold(),
            0x3 => self.set_underline(),
            0x4 => self.unset_underline(),
            0x5 => self.set_blink(),
            0x6 => self.unset_blink(),
            0x7 => self.set_italics(),
            0x8 => self.unset_italics(),
            0x9 => self.set_strikethrough(),
            0xA => self.unset_strikethrough(),
            _ => {}
        }

        let x = ((address & 0x0000FFFF) % self.width as Word) + 1;
        let y = ((address & 0x0000FFFF) / self.height as Word) + 1;
        self.move_to(x * 2, y);

        let character = String::from_utf8((word & 0x0000FFFF).to_le_bytes().to_vec()).expect(
            format!(
                "[VM] Failed to convert Word to UTF-8 character: {:?}",
                word.to_le_bytes()
            )
            .as_str(),
        );

        match io::stdout().write(format!("{}", character).as_bytes()) {
            Ok(_) => (),
            Err(_) => panic!("[VM] Failed to write to stdout"),
        }
    }
}
