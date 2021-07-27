use macros::reg;

use crate::cpu::CPU;

macro_rules! instr {
    ($cpu:ident, w, $op:tt) => {
        let val = $cpu.fetch_word();
        let addr = $cpu.fetch_word();

        if $cpu.get_reg(reg!("acc")) $op val {
            $cpu.set_reg(reg!("pc"), addr);
        }
    };

    ($cpu:ident, r, $op:tt) => {
        let r_addr = $cpu.fetch_word();
        let r_val = $cpu.get_reg(r_addr);

        let addr = $cpu.fetch_word();

        if $cpu.get_reg(reg!("acc")) $op r_val {
            $cpu.set_reg(reg!("pc"), addr);
        }
    };

    ($cpu:ident, rw, $op:tt) => {
        let r_addr = $cpu.fetch_word();
        let r_val = $cpu.get_reg(r_addr);

        let val = $cpu.fetch_word();

        let addr = $cpu.fetch_word();

        if r_val $op val {
            $cpu.set_reg(reg!("pc"), addr);
        }
    };

    ($cpu:ident, rr, $op:tt) => {
        let r1_addr = $cpu.fetch_word();
        let r1_val = $cpu.get_reg(r1_addr);

        let r2_addr = $cpu.fetch_word();
        let r2_val = $cpu.get_reg(r2_addr);

        let addr = $cpu.fetch_word();

        if r1_val $op r2_val {
            $cpu.set_reg(reg!("pc"), addr);
        }
    };
}

/// ## BRBS FLAG_Z, 0xAF
/// If the flag Z is set, jump to 0xAF
#[inline(always)]
pub fn brbs(cpu: &mut CPU) {
    let flag = cpu.fetch_byte();
    let addr = cpu.fetch_word();
    if cpu.get_status_flag(flag) {
        cpu.set_reg(reg!("pc"), addr);
    }
}

/// ## BRBC FLAG_Z, 0xAF
/// If the flag Z is clear, jump to 0xAF
#[inline(always)]
pub fn brbc(cpu: &mut CPU) {
    let flag = cpu.fetch_byte();
    let addr = cpu.fetch_word();
    if !cpu.get_status_flag(flag) {
        cpu.set_reg(reg!("pc"), addr);
    }
}

/// ## BREQ 0x1234, 0x5
/// Jump to 0x5 if acc does equal 0x1234
#[inline(always)]
pub fn breq(cpu: &mut CPU) {
    instr!(cpu, w, ==);
}

/// ## BREQR r1, 0x5
/// Jump to 0x5 if acc does equal register r1
#[inline(always)]
pub fn breqr(cpu: &mut CPU) {
    instr!(cpu, r, ==);
}

/// ## BREQRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 does equal 0x1234
#[inline(always)]
pub fn breqrw(cpu: &mut CPU) {
    instr!(cpu, rw, ==);
}

/// ## BREQRR r1, r2, 0x5
/// Jump to 0x5 if register r1 does equal register r2
#[inline(always)]
pub fn breqrr(cpu: &mut CPU) {
    instr!(cpu, rr, ==);
}

/// ## BRNQ 0x1234, 0x5
/// Jump to 0x5 if acc does not equal 0x1234
#[inline(always)]
pub fn brnq(cpu: &mut CPU) {
    instr!(cpu, w, !=);
}

/// ## BRNQR r1, 0x5
/// Jump to 0x5 if acc does not equal register r1
#[inline(always)]
pub fn brnqr(cpu: &mut CPU) {
    instr!(cpu, r, !=);
}

/// ## BRNQRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 does not equal 0x1234
#[inline(always)]
pub fn brnqrw(cpu: &mut CPU) {
    instr!(cpu, rw, !=);
}

/// ## BRNQRR r1, r2, 0x5
/// Jump to 0x5 if register r1 does not equal register r2
#[inline(always)]
pub fn brnqrr(cpu: &mut CPU) {
    instr!(cpu, rr, !=);
}

/// ## BRLT 0x1234, 0x5
/// Jump to 0x5 if acc is less than 0x1234
#[inline(always)]
pub fn brlt(cpu: &mut CPU) {
    instr!(cpu, w, <);
}

/// ## BRLTR r1, 0x5
/// Jump to 0x5 if acc is less than register r1
#[inline(always)]
pub fn brltr(cpu: &mut CPU) {
    instr!(cpu, r, <);
}

/// ## BRLTRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is less than 0x1234
#[inline(always)]
pub fn brltrw(cpu: &mut CPU) {
    instr!(cpu, rw, <);
}

/// ## BRLTRR r1, r2, 0x5
/// Jump to 0x5 if register r1 is less than register r2
#[inline(always)]
pub fn brltrr(cpu: &mut CPU) {
    instr!(cpu, rr, <);
}

/// ## BRGT 0x1234, 0x5
/// Jump to 0x5 if acc is greater than 0x1234
#[inline(always)]
pub fn brgt(cpu: &mut CPU) {
    instr!(cpu, w, >);
}

/// ## BRGTR r1, 0x5
/// Jump to 0x5 if acc is greater than register r1
#[inline(always)]
pub fn brgtr(cpu: &mut CPU) {
    instr!(cpu, r, >);
}

/// ## BRGTRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is greater than 0x1234
#[inline(always)]
pub fn brgtrw(cpu: &mut CPU) {
    instr!(cpu, rw, >);
}

/// ## BRGTRR r1, r2, 0x5
/// Jump to 0x5 if register r1 is greater than register r2
#[inline(always)]
pub fn brgtrr(cpu: &mut CPU) {
    instr!(cpu, rr, >);
}

/// ## BRLTE 0x1234, 0x5
/// Jump to 0x5 if acc is less than or equal 0x1234
#[inline(always)]
pub fn brlte(cpu: &mut CPU) {
    instr!(cpu, w, <=);
}

/// ## BRLTER r1, 0x5
/// Jump to 0x5 if acc is less than or equal register r1
#[inline(always)]
pub fn brlter(cpu: &mut CPU) {
    instr!(cpu, r, <=);
}

/// ## BRLTERW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is less than or equal 0x1234
#[inline(always)]
pub fn brlterw(cpu: &mut CPU) {
    instr!(cpu, rw, <=);
}

/// ## BRLTERR r1, r2, 0x5
/// Jump to 0x5 if register r1 is less than register or equal r2
#[inline(always)]
pub fn brlterr(cpu: &mut CPU) {
    instr!(cpu, rr, <=);
}

/// ## BRGTE 0x1234, 0x5
/// Jump to 0x5 if acc is greater than or equal 0x1234
#[inline(always)]
pub fn brgte(cpu: &mut CPU) {
    instr!(cpu, w, >=);
}

/// ## BRGTER r1, 0x5
/// Jump to 0x5 if acc is greater than register or equal r1
#[inline(always)]
pub fn brgter(cpu: &mut CPU) {
    instr!(cpu, r, >=);
}

/// ## BRGTERW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is greater than or equal 0x1234
#[inline(always)]
pub fn brgterw(cpu: &mut CPU) {
    instr!(cpu, rw, >=);
}

/// ## BRGTERR r1, r2, 0x5
/// Jump to 0x5 if register r1 is greater than or equal register r2
#[inline(always)]
pub fn brgterr(cpu: &mut CPU) {
    instr!(cpu, rr, >=);
}
