use crate::{
    device::Device,
    memory::{Byte, Memory, MemoryMapper, Word},
};
use macros::reg;

use super::{instr_codes::*, instructions::*};

pub struct CPU {
    pub memory_mapper: MemoryMapper,
    registers: Memory,
    stackframe_size: Word,
    halt_signal: bool,
}

#[allow(dead_code)]
impl CPU {
    pub fn new(memory_mapper: MemoryMapper) -> CPU {
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
    pub fn get_status_flag(&self, n: Byte) -> bool {
        self.get_reg(reg!("sr")) & (1u32.wrapping_shl(n as Word)) != 0
    }

    /// Gets the val of the register with the given addr.
    pub fn get_reg(&self, addr: Word) -> Word {
        self.registers.get_word(addr)
    }

    /// Sets the val of the register with the given addr.
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
        match instr {
            instr_codes::HALT => {
                self.halt_signal = true;
            }
            instr_codes::NOP => {}
            instr_codes::MOVR => movr(self),
            instr_codes::MOVM => movm(self),
            instr_codes::MOVRR => movrr(self),
            instr_codes::MOVRM => movrm(self),
            instr_codes::MOVMR => movmr(self),
            instr_codes::MOVRPR => movrpr(self),
            instr_codes::MOVROR => movror(self),
            instr_codes::POP => pop(self),
            instr_codes::PUSH => push(self),
            instr_codes::PUSHR => pushr(self),
            instr_codes::ADD => add(self),
            instr_codes::ADDR => addr(self),
            instr_codes::SUB => sub(self),
            instr_codes::SUBWR => subwr(self),
            instr_codes::SUBR => subr(self),
            instr_codes::MULT => mult(self),
            instr_codes::MULTR => multr(self),
            instr_codes::DIV => div(self),
            instr_codes::DIVWR => divwr(self),
            instr_codes::DIVR => divr(self),
            instr_codes::INC => inc(self),
            instr_codes::DEC => dec(self),
            instr_codes::LSF => lsf(self),
            instr_codes::LSFR => lsfr(self),
            instr_codes::RSF => rsf(self),
            instr_codes::RSFR => rsfr(self),
            instr_codes::WLSF => wlsf(self),
            instr_codes::WLSFR => wlsfr(self),
            instr_codes::WRSF => wrsf(self),
            instr_codes::WRSFR => wrsfr(self),
            instr_codes::AND => and(self),
            instr_codes::ANDR => andr(self),
            instr_codes::OR => or(self),
            instr_codes::ORR => orr(self),
            instr_codes::XOR => xor(self),
            instr_codes::XORR => xorr(self),
            instr_codes::NOT => not(self),
            instr_codes::BRBS => brbs(self),
            instr_codes::BRBC => brbc(self),
            instr_codes::BREQ => breq(self),
            instr_codes::BREQR => breqr(self),
            instr_codes::BREQRW => breqrw(self),
            instr_codes::BREQRR => breqrr(self),
            instr_codes::BRNQ => brnq(self),
            instr_codes::BRNQR => brnqr(self),
            instr_codes::BRNQRW => brnqrw(self),
            instr_codes::BRNQRR => brnqrr(self),
            instr_codes::BRLT => brlt(self),
            instr_codes::BRLTR => brltr(self),
            instr_codes::BRLTRW => brltrw(self),
            instr_codes::BRLTRR => brltrr(self),
            instr_codes::BRGT => brgt(self),
            instr_codes::BRGTR => brgtr(self),
            instr_codes::BRGTRW => brgtrw(self),
            instr_codes::BRGTRR => brgtrr(self),
            instr_codes::BRLTE => brlte(self),
            instr_codes::BRLTER => brlter(self),
            instr_codes::BRLTERW => brlterw(self),
            instr_codes::BRLTERR => brlterr(self),
            instr_codes::BRGTE => brgte(self),
            instr_codes::BRGTER => brgter(self),
            instr_codes::BRGTERW => brgterw(self),
            instr_codes::BRGTERR => brgterr(self),
            instr_codes::JMP => jmp(self),
            instr_codes::CALL => call(self),
            instr_codes::CALLR => callr(self),
            instr_codes::RET => ret(self),
            _ => {
                panic!("[CPU] No such instruction: '0x{:02X}'", instr);
            }
        }
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
