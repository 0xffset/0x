pub mod instruction_codes {
    use crate::memory::Byte;

	pub const MOVI_WORD_REG: Byte = 0x10;
	pub const ADD_REG_REG: Byte = 0x11;
}