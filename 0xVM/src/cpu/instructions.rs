pub mod instruction_codes {
    use crate::memory::Byte;

	pub const HALT: Byte = 0x00;
	pub const NOP: Byte = 0x01;

	// MOV
	pub const MOV_WORD_REG: Byte = 0x10;
	pub const MOV_WORD_MEM: Byte = 0x11;
	pub const MOV_REG_REG: Byte= 0x12;
	pub const MOV_REG_MEM: Byte= 0x13;
	pub const MOV_MEM_REG: Byte= 0x14;

	// Arithmetic
	pub const ADD_REG_REG: Byte = 0x20;
}