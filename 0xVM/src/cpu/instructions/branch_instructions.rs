use macros::reg;

use crate::cpu::VM;

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
#[inline]
#[allow(non_snake_case)]
pub fn BRBS(cpu: &mut VM) {
    let flag = cpu.fetch_byte();
    let addr = cpu.fetch_word();
    if cpu.get_status_flag(flag) {
        cpu.set_reg(reg!("pc"), addr);
    }
}

/// ## BRBC FLAG_Z, 0xAF
/// If the flag Z is clear, jump to 0xAF
#[inline]
#[allow(non_snake_case)]
pub fn BRBC(cpu: &mut VM) {
    let flag = cpu.fetch_byte();
    let addr = cpu.fetch_word();
    if !cpu.get_status_flag(flag) {
        cpu.set_reg(reg!("pc"), addr);
    }
}

/// ## BREQ 0x1234, 0x5
/// Jump to 0x5 if acc does equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BREQ(cpu: &mut VM) {
    instr!(cpu, w, ==);
}

/// ## BREQR r1, 0x5
/// Jump to 0x5 if acc does equal register r1
#[inline]
#[allow(non_snake_case)]
pub fn BREQR(cpu: &mut VM) {
    instr!(cpu, r, ==);
}

/// ## BREQRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 does equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BREQRW(cpu: &mut VM) {
    instr!(cpu, rw, ==);
}

/// ## BREQRR r1, r2, 0x5
/// Jump to 0x5 if register r1 does equal register r2
#[inline]
#[allow(non_snake_case)]
pub fn BREQRR(cpu: &mut VM) {
    instr!(cpu, rr, ==);
}

/// ## BRNQ 0x1234, 0x5
/// Jump to 0x5 if acc does not equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRNQ(cpu: &mut VM) {
    instr!(cpu, w, !=);
}

/// ## BRNQR r1, 0x5
/// Jump to 0x5 if acc does not equal register r1
#[inline]
#[allow(non_snake_case)]
pub fn BRNQR(cpu: &mut VM) {
    instr!(cpu, r, !=);
}

/// ## BRNQRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 does not equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRNQRW(cpu: &mut VM) {
    instr!(cpu, rw, !=);
}

/// ## BRNQRR r1, r2, 0x5
/// Jump to 0x5 if register r1 does not equal register r2
#[inline]
#[allow(non_snake_case)]
pub fn BRNQRR(cpu: &mut VM) {
    instr!(cpu, rr, !=);
}

/// ## BRLT 0x1234, 0x5
/// Jump to 0x5 if acc is less than 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRLT(cpu: &mut VM) {
    instr!(cpu, w, <);
}

/// ## BRLTR r1, 0x5
/// Jump to 0x5 if acc is less than register r1
#[inline]
#[allow(non_snake_case)]
pub fn BRLTR(cpu: &mut VM) {
    instr!(cpu, r, <);
}

/// ## BRLTRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is less than 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRLTRW(cpu: &mut VM) {
    instr!(cpu, rw, <);
}

/// ## BRLTRR r1, r2, 0x5
/// Jump to 0x5 if register r1 is less than register r2
#[inline]
#[allow(non_snake_case)]
pub fn BRLTRR(cpu: &mut VM) {
    instr!(cpu, rr, <);
}

/// ## BRGT 0x1234, 0x5
/// Jump to 0x5 if acc is greater than 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRGT(cpu: &mut VM) {
    instr!(cpu, w, >);
}

/// ## BRGTR r1, 0x5
/// Jump to 0x5 if acc is greater than register r1
#[inline]
#[allow(non_snake_case)]
pub fn BRGTR(cpu: &mut VM) {
    instr!(cpu, r, >);
}

/// ## BRGTRW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is greater than 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRGTRW(cpu: &mut VM) {
    instr!(cpu, rw, >);
}

/// ## BRGTRR r1, r2, 0x5
/// Jump to 0x5 if register r1 is greater than register r2
#[inline]
#[allow(non_snake_case)]
pub fn BRGTRR(cpu: &mut VM) {
    instr!(cpu, rr, >);
}

/// ## BRLTE 0x1234, 0x5
/// Jump to 0x5 if acc is less than or equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRLTE(cpu: &mut VM) {
    instr!(cpu, w, <=);
}

/// ## BRLTER r1, 0x5
/// Jump to 0x5 if acc is less than or equal register r1
#[inline]
#[allow(non_snake_case)]
pub fn BRLTER(cpu: &mut VM) {
    instr!(cpu, r, <=);
}

/// ## BRLTERW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is less than or equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRLTERW(cpu: &mut VM) {
    instr!(cpu, rw, <=);
}

/// ## BRLTERR r1, r2, 0x5
/// Jump to 0x5 if register r1 is less than register or equal r2
#[inline]
#[allow(non_snake_case)]
pub fn BRLTERR(cpu: &mut VM) {
    instr!(cpu, rr, <=);
}

/// ## BRGTE 0x1234, 0x5
/// Jump to 0x5 if acc is greater than or equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRGTE(cpu: &mut VM) {
    instr!(cpu, w, >=);
}

/// ## BRGTER r1, 0x5
/// Jump to 0x5 if acc is greater than register or equal r1
#[inline]
#[allow(non_snake_case)]
pub fn BRGTER(cpu: &mut VM) {
    instr!(cpu, r, >=);
}

/// ## BRGTERW r1, 0x1234, 0x5
/// Jump to 0x5 if register r1 is greater than or equal 0x1234
#[inline]
#[allow(non_snake_case)]
pub fn BRGTERW(cpu: &mut VM) {
    instr!(cpu, rw, >=);
}

/// ## BRGTERR r1, r2, 0x5
/// Jump to 0x5 if register r1 is greater than or equal register r2
#[inline]
#[allow(non_snake_case)]
pub fn BRGTERR(cpu: &mut VM) {
    instr!(cpu, rr, >=);
}
