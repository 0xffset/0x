use crate::{
    device::Device,
    memory::{Byte, Memory, MemoryMapper, Word},
};
use macros::reg;

use super::instructions::*;

macro_rules! generate_execute {
    ($self:ident, $instr:ident, $([$(($op:literal, $instr_func:ident)),+]),+) => {
        match $instr {
            0xFF => $self.halt_signal = true,
            0x00 => {},
            $($($op => $instr_func($self),)*)*
            _ => panic!("[CPU] No such instruction: '0x{:02X}'", $instr)
        }
    };
}

pub struct CPU {
    pub memory_mapper: MemoryMapper,
    registers: Memory,
    stackframe_size: Word,
    halt_signal: bool,
}

#[allow(dead_code)]
impl CPU {
    pub fn new(memory_mapper: MemoryMapper) -> Self {
        let mut ret = CPU {
            memory_mapper,
            registers: Memory::new((crate::REGISTER_COUNT * 4) as u32),
            stackframe_size: 0,
            halt_signal: false,
        };

        // -4 because 4 bytes to store a 32-Bit addr
        // TODO: change this to not hard coded
        ret.set_reg(reg!("sp"), 0xFFFF - 4);
        ret.set_reg(reg!("fp"), 0xFFFF - 4);

        ret
    }

    pub fn update_sr(&mut self, pre: Word, post: Word) {
        let sr_addr = reg!("sr");
        if post == 0 {
            self.registers.or_set_byte(sr_addr, 0x01);
        } else {
            self.registers.and_set_byte(sr_addr, 0xFE);
        }

        if pre > post {
            self.registers.or_set_byte(sr_addr, 0x02);
        } else {
            self.registers.and_set_byte(sr_addr, 0xFD);
        }
    }

    /// Gets status flag at n
    ///
    /// # Example:
    ///
    /// ```
    /// get_status_flag(1);
    /// // returns `true` if bit 1 is set
    /// ```
    #[inline]
    pub fn get_status_flag(&self, n: Byte) -> bool {
        self.get_reg(reg!("sr")) & (1u32.wrapping_shl(n as Word)) != 0
    }

    /// Gets the val of the register with the given addr.
    #[inline]
    pub fn get_reg(&self, addr: Word) -> Word {
        self.registers.get_word(addr)
    }

    /// Sets the val of the register with the given addr.
    #[inline]
    pub fn set_reg(&mut self, addr: Word, val: Word) {
        self.registers.set_word(addr, val);
    }

    /// Fetches the next byte from memory and increments the program counter.
    pub fn fetch_byte(&mut self) -> Byte {
        let next_instr_addr = self.get_reg(reg!("pc"));
        self.set_reg(reg!("pc"), next_instr_addr + 1);

        self.memory_mapper.get_byte(next_instr_addr)
    }

    /// Fetches the next word from memory and increments the program counter.
    pub fn fetch_word(&mut self) -> Word {
        let next_instr_addr = self.get_reg(reg!("pc"));
        self.set_reg(reg!("pc"), next_instr_addr + 4);

        self.memory_mapper.get_word(next_instr_addr)
    }

    /// Pushes onto stack and increments stackframe size
    pub fn push(&mut self, val: Word) {
        let sp_addr = self.get_reg(reg!("sp"));
        self.memory_mapper.set_word(sp_addr, val);
        self.set_reg(reg!("sp"), sp_addr - 4);

        self.stackframe_size += 4;
    }

    /// Pops from the stack and decrements stackframe size
    pub fn pop(&mut self) -> Word {
        let next_sp_addr = self.get_reg(reg!("sp")) + 4;
        self.set_reg(reg!("sp"), next_sp_addr);

        self.stackframe_size -= 4;

        return self.memory_mapper.get_word(next_sp_addr);
    }

    /// Push state onto stack after CALL
    pub fn push_state(&mut self) {
        for i in 0..8 {
            self.push(self.get_reg(i * 4));
        }

        self.push(self.get_reg(reg!("pc")));
        self.push(self.stackframe_size + 4);

        self.set_reg(reg!("fp"), self.get_reg(reg!("sp")));
        self.stackframe_size = 0;
    }

    /// Pop state from stack after RET
    pub fn pop_state(&mut self) {
        let fp_addr = self.get_reg(reg!("fp"));
        self.set_reg(reg!("sp"), fp_addr);

        // bugfix where the stackframe is 0 but we need to pop the stackframe size
        self.stackframe_size += 4;
        self.stackframe_size = self.pop();

        let pc_addr = self.pop();
        self.set_reg(reg!("pc"), pc_addr);

        for i in (0..8).rev() {
            let gp_reg_val = self.pop();
            self.set_reg(i * 4, gp_reg_val);
        }

        let arg_count = self.pop();
        for _ in 0..arg_count {
            self.pop();
        }

        self.set_reg(reg!("fp"), fp_addr + self.stackframe_size);
    }

    fn execute(&mut self, instr: Byte) {
        generate_execute!(
            self,
            instr,
            // move instructions
            [
                (0x10, MOVR),
                (0x11, MOVM),
                (0x12, MOVRR),
                (0x13, MOVRM),
                (0x14, MOVMR),
                (0x17, MOVRPR),
                (0x18, MOVROR),
                (0x05, POP),
                (0x15, PUSH),
                (0x16, PUSHR)
            ],
            // sub routine instructions
            [(0x01, JMP), (0x02, CALL), (0x03, CALLR), (0x04, RET)],
            // arithmetic instructions
            [
                (0x20, ADD),
                (0x21, ADDR),
                (0x22, SUB),
                (0x23, SUBWR),
                (0x24, SUBR),
                (0x25, MULT),
                (0x26, MULTR),
                (0x27, DIV),
                (0x28, DIVWR),
                (0x29, DIVR),
                (0x2A, INC),
                (0x2B, DEC)
            ],
            // bitwise instructions
            [
                (0x50, LSF),
                (0x51, LSFR),
                (0x52, RSF),
                (0x53, RSFR),
                (0x54, WLSF),
                (0x55, WLSFR),
                (0x56, WRSF),
                (0x57, WRSFR),
                (0x58, AND),
                (0x59, ANDR),
                (0x5A, OR),
                (0x5B, ORR),
                (0x5C, XOR),
                (0x5D, XORR),
                (0x5E, NOT)
            ],
            // conditional instructions
            [
                (0x30, BRBS),
                (0x31, BRBC),
                (0x32, BREQ),
                (0x33, BREQR),
                (0x34, BREQRW),
                (0x35, BREQRR),
                (0x36, BRNQ),
                (0x37, BRNQR),
                (0x38, BRNQRW),
                (0x39, BRNQRR),
                (0x3A, BRLT),
                (0x3B, BRLTR),
                (0x3C, BRLTRW),
                (0x3D, BRLTRR),
                (0x3E, BRGT),
                (0x3F, BRGTR),
                (0x40, BRGTRW),
                (0x41, BRGTRR),
                (0x42, BRLTE),
                (0x43, BRLTER),
                (0x44, BRLTERW),
                (0x45, BRLTERR),
                (0x46, BRGTE),
                (0x47, BRGTER),
                (0x48, BRGTERW),
                (0x49, BRGTERR)
            ]
        );
    }

    /// Prints a view of all registers to the console
    pub fn debug(&self) {
        for (name, addr) in crate::REGISTERS {
            println!("{:<4}: 0x{:08X}", name, self.get_reg(*addr));
        }
        println!();
    }

    /// Prints a view of a region of the memory to the console
    fn view_memory_at(&self, addr: Word, n: Word) {
        let mut mem_snapshot: Vec<Byte> = Vec::new();
        // TODO

        let max_addr = addr + n;

        for i in addr..max_addr {
            mem_snapshot.push(self.memory_mapper.get_byte(i));
        }

        for (offset, byte) in mem_snapshot.iter().enumerate() {
            if offset % 16 == 0 {
                print!("\n0x{:08X}:", addr as usize + offset);
            }
            print!(" 0x{:02X}", byte);
        }
        println!();
    }

    /// Progresses the program
    fn step(&mut self) {
        let instr = self.fetch_byte();
        self.execute(instr);
    }

    pub fn run_debug(&mut self, addr: Word, n: Word) {
        use std::io;

        while !self.halt_signal {
            self.step();
            self.debug();
            self.view_memory_at(addr, n);

            io::stdin().read_line(&mut String::new()).unwrap();
        }
    }

    pub fn run(&mut self) {
        while !self.halt_signal {
            self.step();
        }
    }
}
