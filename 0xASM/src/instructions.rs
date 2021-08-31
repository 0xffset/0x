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
    pub const MOVRPR: (Byte, usize) = (0x17, 2);
    pub const MOVROR: (Byte, usize) = (0x18, 3);
    pub const LOAD: (Byte, usize) = (0x19, 3);
    pub const LOADR: (Byte, usize) = (0x1A, 3);
    pub const LOADM: (Byte, usize) = (0x1B, 3);
    pub const STORE: (Byte, usize) = (0x1C, 3);
    pub const STORER: (Byte, usize) = (0x1D, 3);
    pub const STOREM: (Byte, usize) = (0x1E, 3);

    // Stack
    pub const POP: (Byte, usize) = (0x05, 1);
    pub const PUSH: (Byte, usize) = (0x15, 1);
    pub const PUSHR: (Byte, usize) = (0x16, 1);

    // Subroutines
    pub const JMP: (Byte, usize) = (0x01, 1);
    pub const CALL: (Byte, usize) = (0x02, 1);
    pub const CALLR: (Byte, usize) = (0x03, 1);
    pub const RET: (Byte, usize) = (0x04, 0);

    // Arithmetic
    pub const ADD: (Byte, usize) = (0x20, 2);
    pub const ADDR: (Byte, usize) = (0x21, 2);
    pub const SUB: (Byte, usize) = (0x22, 2);
    pub const SUBWR: (Byte, usize) = (0x23, 2);
    pub const SUBR: (Byte, usize) = (0x24, 2);
    pub const MULT: (Byte, usize) = (0x25, 2);
    pub const MULTR: (Byte, usize) = (0x26, 2);
    pub const DIV: (Byte, usize) = (0x27, 2);
    pub const DIVWR: (Byte, usize) = (0x28, 2);
    pub const DIVR: (Byte, usize) = (0x29, 2);
    pub const INC: (Byte, usize) = (0x2A, 1);
    pub const DEC: (Byte, usize) = (0x2B, 1);

    // Bitwise
    pub const LSF: (Byte, usize) = (0x50, 2);
    pub const LSFR: (Byte, usize) = (0x51, 2);
    pub const RSF : (Byte, usize) = (0x52, 2);
    pub const RSFR: (Byte, usize) = (0x53, 2);
    pub const WLSF: (Byte, usize) = (0x54, 2);
    pub const WLSFR: (Byte, usize) = (0x55, 2);
    pub const WRSF: (Byte, usize) = (0x56, 2);
    pub const WRSFR: (Byte, usize) = (0x57, 2);
    pub const AND: (Byte, usize) = (0x58, 2);
    pub const ANDR: (Byte, usize) = (0x59, 2);
    pub const OR: (Byte, usize) = (0x5A, 2);
    pub const ORR: (Byte, usize) = (0x5B, 2);
    pub const XOR: (Byte, usize) = (0x5C, 2);
    pub const XORR: (Byte, usize) = (0x5D, 2);
    pub const NOT: (Byte, usize) = (0x5E, 1);

    // Conditional jumps
    pub const BRBS: (Byte, usize) = (0x30, 2);
    pub const BRBC: (Byte, usize) = (0x31, 2);
    pub const BREQ: (Byte, usize) = (0x32, 2);
    pub const BREQR: (Byte, usize) = (0x33, 2);
    pub const BREQRW: (Byte, usize) = (0x34, 3);
    pub const BREQRR: (Byte, usize) = (0x35, 3);
    pub const BRNQ: (Byte, usize) = (0x36, 2);
    pub const BRNQR: (Byte, usize) = (0x37, 2);
    pub const BRNQRW: (Byte, usize) = (0x38, 3);
    pub const BRNQRR: (Byte, usize) = (0x39, 3);
    pub const BRLT: (Byte, usize) = (0x3A, 2);
    pub const BRLTR: (Byte, usize) = (0x3B, 2);
    pub const BRLTRW: (Byte, usize) = (0x3C, 3);
    pub const BRLTRR: (Byte, usize) = (0x3D, 3);
    pub const BRGT: (Byte, usize) = (0x3E, 2);
    pub const BRGTR: (Byte, usize) = (0x3F, 2);
    pub const BRGTRW: (Byte, usize) = (0x40, 3);
    pub const BRGTRR: (Byte, usize) = (0x41, 3);
    pub const BRLTE: (Byte, usize) = (0x42, 2);
    pub const BRLTER: (Byte, usize) = (0x43, 2);
    pub const BRLTERW: (Byte, usize) = (0x44, 3);
    pub const BRLTERR: (Byte, usize) = (0x45, 3);
    pub const BRGTE: (Byte, usize) = (0x46, 2);
    pub const BRGTER: (Byte, usize) = (0x47, 2);
    pub const BRGTERW: (Byte, usize) = (0x48, 3);
    pub const BRGTERR: (Byte, usize) = (0x49, 3);
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
        "MOVRPR" => Some(MOVRPR),
        "MOVROR" => Some(MOVROR),
        "LOAD" => Some(LOAD),
        "LOADR" => Some(LOADR),
        "LOADM" => Some(LOADM),
        "STORE" => Some(STORE),
        "STORER" => Some(STORER),
        "STOREM" => Some(STOREM),

        "POP" => Some(POP),
        "PUSH" => Some(PUSH),
        "PUSHR" => Some(PUSHR),

        "JMP" => Some(JMP),
        "CALL" => Some(CALL),
        "CALLR" => Some(CALLR),
        "RET" => Some(RET),

        "ADD" => Some(ADD),
        "ADDR" => Some(ADDR),
        "SUB" => Some(SUB),
        "SUBWR" => Some(SUBWR),
        "SUBR" => Some(SUBR),
        "MULT" => Some(MULT),
        "MULTR" => Some(MULTR),
        "DIV" => Some(DIV),
        "DIVWR" => Some(DIVWR),
        "DIVR" => Some(DIVR),
        "INC" => Some(INC),
        "DEC" => Some(DEC),

        "LSF" => Some(LSF),
        "LSFR" => Some(LSFR),
        "RSF" => Some(RSF),
        "RSFR" => Some(RSFR),
        "WLSF" => Some(WLSF),
        "WLSFR" => Some(WLSFR),
        "WRSF" => Some(WRSF),
        "WRSFR" => Some(WRSFR),
        "AND" => Some(AND),
        "ANDR" => Some(ANDR),
        "OR" => Some(OR),
        "ORR" => Some(ORR),
        "XOR" => Some(XOR),
        "XORR" => Some(XORR),
        "NOT" => Some(NOT),

        "BRBS" => Some(BRBS),
        "BRBC" => Some(BRBC),
        "BREQ" => Some(BREQ),
        "BREQR" => Some(BREQR),
        "BREQRW" => Some(BREQRW),
        "BREQRR" => Some(BREQRR),
        "BRNQ" => Some(BRNQ),
        "BRNQR" => Some(BRNQR),
        "BRNQRW" => Some(BRNQRW),
        "BRNQRR" => Some(BRNQRR),
        "BRLT" => Some(BRLT),
        "BRLTR" => Some(BRLTR),
        "BRLTRW" => Some(BRLTRW),
        "BRLTRR" => Some(BRLTRR),
        "BRGT" => Some(BRGT),
        "BRGTR" => Some(BRGTR),
        "BRGTRW" => Some(BRGTRW),
        "BRGTRR" => Some(BRGTRR),
        "BRLTE" => Some(BRLTE),
        "BRLTER" => Some(BRLTER),
        "BRLTERW" => Some(BRLTERW),
        "BRLTERR" => Some(BRLTERR),
        "BRGTE" => Some(BRGTE),
        "BRGTER" => Some(BRGTER),
        "BRGTERW" => Some(BRGTERW),
        "BRGTERR" => Some(BRGTERR),
        _ => None,
    }
}