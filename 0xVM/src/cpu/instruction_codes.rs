pub mod instruction_codes {
    use crate::memory::Byte;

    pub const HALT: Byte = 0xFF;
    pub const NOP: Byte = 0x00;

    // MOV
    pub const MOVR: Byte = 0x10;
    pub const MOVM: Byte = 0x11;
    pub const MOVRR: Byte = 0x12;
    pub const MOVRM: Byte = 0x13;
    pub const MOVMR: Byte = 0x14;
    pub const MOVRPR : Byte = 0x17;
    pub const MOVROR : Byte = 0x18;

	pub const POP: Byte = 0x05;
    pub const PUSH: Byte = 0x15;
    pub const PUSHR: Byte = 0x16;

	pub const CALL: Byte = 0x02;
	pub const CALLR: Byte = 0x03;
	pub const RET: Byte = 0x04;

    // Arithmetic
    pub const ADD: Byte = 0x20;
    pub const ADDR: Byte = 0x21;

    // Conditional jumps
    pub const BRBS: Byte = 0x30;
    pub const BRBC: Byte = 0x31;
    pub const BREQ: Byte = 0x32;
    pub const BRNQ: Byte = 0x33;
}
