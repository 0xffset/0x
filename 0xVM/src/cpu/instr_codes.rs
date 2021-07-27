pub mod instr_codes {
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

    pub const JMP: Byte = 0x01;
	pub const CALL: Byte = 0x02;
	pub const CALLR: Byte = 0x03;
	pub const RET: Byte = 0x04;

    // Arithmetic
    pub const ADD: Byte = 0x20;
    pub const ADDR: Byte = 0x21;
    pub const SUB: Byte = 0x22;
    pub const SUBWR: Byte = 0x23;
    pub const SUBR: Byte = 0x24;
    pub const MULT: Byte = 0x25;
    pub const MULTR: Byte = 0x26;
    pub const DIV: Byte = 0x27;
    pub const DIVWR: Byte = 0x28;
    pub const DIVR: Byte = 0x29;
    pub const INC: Byte = 0x2A;
    pub const DEC: Byte = 0x2B;

    // Bitwise
    pub const LSF: Byte = 0x50;
    pub const LSFR: Byte = 0x51;
    pub const RSF : Byte = 0x52;
    pub const RSFR: Byte = 0x53;
    pub const WLSF: Byte = 0x54;
    pub const WLSFR: Byte = 0x55;
    pub const WRSF: Byte = 0x56;
    pub const WRSFR: Byte = 0x57;
    pub const AND: Byte = 0x58;
    pub const ANDR: Byte = 0x59;
    pub const OR: Byte = 0x5A;
    pub const ORR: Byte = 0x5B;
    pub const XOR: Byte = 0x5C;
    pub const XORR: Byte = 0x5D;
    pub const NOT: Byte = 0x5E;

    // Conditional jumps
    pub const BRBS: Byte = 0x30;
    pub const BRBC: Byte = 0x31;
    pub const BREQ: Byte = 0x32;
    pub const BREQR: Byte = 0x33;
    pub const BREQRW: Byte = 0x34;
    pub const BREQRR: Byte = 0x35;
    pub const BRNQ: Byte = 0x36;
    pub const BRNQR: Byte = 0x37;
    pub const BRNQRW: Byte = 0x38;
    pub const BRNQRR: Byte = 0x39;
    pub const BRLT: Byte = 0x3A;
    pub const BRLTR: Byte = 0x3B;
    pub const BRLTRW: Byte = 0x3C;
    pub const BRLTRR: Byte = 0x3D;
    pub const BRGT: Byte = 0x3E;
    pub const BRGTR: Byte = 0x3F;
    pub const BRGTRW: Byte = 0x40;
    pub const BRGTRR: Byte = 0x41;
    pub const BRLTE: Byte = 0x42;
    pub const BRLTER: Byte = 0x43;
    pub const BRLTERW: Byte = 0x44;
    pub const BRLTERR: Byte = 0x45;
    pub const BRGTE: Byte = 0x46;
    pub const BRGTER: Byte = 0x47;
    pub const BRGTERW: Byte = 0x48;
    pub const BRGTERR: Byte = 0x49;
}
