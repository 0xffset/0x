use crate::memory::{Byte, Word};

pub trait Device {
    fn get_word(&self, _: Word) -> Word {
        panic!("[DEVICE] Device '{}' didn't implement 'get_word()'", std::any::type_name_of_val(self));
    }
    fn set_word(&mut self, _: Word, _: Word) -> () {
        panic!("[DEVICE] Device '{}' didn't implement 'set_word()'", std::any::type_name_of_val(self));
    }

    // mandatory for debugging
    fn get_byte(&self, _: Word) -> Byte;

    fn set_byte(&mut self, _: Word, _: Byte) -> () {
        panic!("[DEVICE] Device '{}' didn't implement 'set_byte()'", std::any::type_name_of_val(self));
    }

    fn get_range(&self, _: Word, _: Word) -> Vec<Byte> {
        panic!("[DEVICE] Device '{}' didn't implement 'get_range()'", std::any::type_name_of_val(self));
    }
    fn set_range(&mut self, _: Word, _: Vec<Byte>) -> () {
        panic!("[DEVICE] Device '{}' didn't implement 'set_range()'", std::any::type_name_of_val(self));
    }

    fn get_buffer(&self) -> Vec<Byte> {
        panic!("[DEVICE] Device '{}' didn't implement 'get_buffer()'", std::any::type_name_of_val(self));
    }
}
