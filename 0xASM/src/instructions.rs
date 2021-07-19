use crate::Byte;

pub mod instruction_codes {
    use crate::Byte;

    pub const HALT: (Byte, usize) = (0xFF, 0);
    pub const NOP: (Byte, usize) = (0x00, 0);

    // MOV
    pub const MOVR: (Byte, usize) = (0x10, 2);
    pub const MOVM: (Byte, usize) = (0x11, 2);
    pub const MOVRR: (Byte, usize) = (0x12, 2);
    pub const MOVRM: (Byte, usize) = (0x13, 2);
    pub const MOVMR: (Byte, usize) = (0x14, 2);

	pub const POP: (Byte, usize) = (0x05, 1);
    pub const PUSH: (Byte, usize) = (0x15, 1);
    pub const PUSHR: (Byte, usize) = (0x16, 1);

	pub const CALL: (Byte, usize) = (0x02, 1);
	pub const CALLR: (Byte, usize) = (0x03, 1);
	pub const RET: (Byte, usize) = (0x04, 0);

    // Arithmetic
    pub const ADD: (Byte, usize) = (0x20, 2);
    pub const ADDR: (Byte, usize) = (0x21, 2);

    // Conditional jumps
    pub const BRBS: (Byte, usize) = (0x30, 2);
    pub const BRBC: (Byte, usize) = (0x31, 2);
    pub const BREQ: (Byte, usize) = (0x32, 2);
    pub const BRNQ: (Byte, usize) = (0x33, 2);
}

pub fn instruction_to_byte(i: &str) -> Option<(Byte, usize)> {
    use instruction_codes::*;

    match i.to_uppercase().as_str() {
        "HALT" => Some(HALT),
        "NOP" => Some(NOP),

        "MOVR" => Some(MOVR),
        "MOVM" => Some(MOVM),
        "MOVRR" => Some(MOVRR),
        "MOVRM" => Some(MOVRM),
        "MOVMR" => Some(MOVMR),

        "POP" => Some(POP),
        "PUSH" => Some(PUSH),
        "PUSHR" => Some(PUSHR),

        "CALL" => Some(CALL),
        "CALLR" => Some(CALLR),
        "RET" => Some(RET),

        "ADD" => Some(ADD),
        "ADDR" => Some(ADDR),

        "BRBS" => Some(BRBS),
        "BRBC" => Some(BRBC),
        "BREQ" => Some(BREQ),
        "BRNQ" => Some(BRNQ),
        _ => None,
    }
}