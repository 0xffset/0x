use crate::memory::{Byte, Word};

pub trait Device {
    fn get_word(&self, _: Word) -> Word {
        panic!("[DEVICE] Device didn't implement 'get_word()'");
    }
    fn set_word(&mut self, _: Word, _: Word) -> () {
        panic!("[DEVICE] Device didn't implement 'set_word()'");
    }

    fn get_byte(&self, _: Word) -> Byte {
        panic!("[DEVICE] Device didn't implement 'get_byte()'");
    }
    fn set_byte(&mut self, _: Word, _: Byte) -> () {
        panic!("[DEVICE] Device didn't implement 'set_byte()'");
    }
}
