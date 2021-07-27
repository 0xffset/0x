use macros::reg;

use crate::cpu::CPU;

macro_rules! instr_helper {
	($cpu:ident, $val1:ident, $f:ident, $val2:ident) => {
        // calculate the result with given function $f, update $destination with result
        // and update status register
        let res = $val1.$f($val2);

        $cpu.set_reg(reg!("acc"), res);

        $cpu.update_sr($val1, res);
    };
}

macro_rules! instr {
    ($cpu:ident, wr, $f:ident) => {
        let val = $cpu.fetch_word();

        let r_addr = $cpu.fetch_word();
        let r_val = $cpu.get_reg(r_addr);

		instr_helper!($cpu, val, $f, r_val);
    };

    ($cpu:ident, rr, $f:ident) => {
        let r1_addr = $cpu.fetch_word();
        let r2_addr = $cpu.fetch_word();

        let r1_val = $cpu.get_reg(r1_addr);
        let r2_val = $cpu.get_reg(r2_addr);

		instr_helper!($cpu, r1_val, $f, r2_val);
    };

    ($cpu:ident, rw, $f:ident) => {
        let r_addr = $cpu.fetch_word();
        let r_val = $cpu.get_reg(r_addr);

        let val = $cpu.fetch_word();

		instr_helper!($cpu, r_val, $f, val);
    };

    ($cpu:ident, cc, $f:ident) => {
        let r_addr = $cpu.fetch_word();
        let r_val = $cpu.get_reg(r_addr);
        let acc = r_val.$f(1);

        $cpu.set_reg(r_addr, acc);

        $cpu.update_sr(r_val, acc);
    };
}

/// ## ADD 0x1234, r1
/// Add 0x1234 to register r1 and store the result in acc
#[inline]
pub fn add(cpu: &mut CPU) {
    instr!(cpu, wr, wrapping_add);
}

/// ## ADDR r1, r2
/// Add register r1 and register r2 and store the result in acc
#[inline]
pub fn addr(cpu: &mut CPU) {
    instr!(cpu, rr, wrapping_add);
}

/// ## SUB r1, 0x1234
/// Subtract 0x1234 from register r1 and store the result in acc
#[inline]
pub fn sub(cpu: &mut CPU) {
    instr!(cpu, rw, wrapping_sub);
}

/// ## SUBWR 0x1234, r1
/// Subtract register r1 from 0x1234 and store the result in acc
#[inline]
pub fn subwr(cpu: &mut CPU) {
    instr!(cpu, wr, wrapping_sub);
}

/// ## SUBR r1, r2
/// Subtract register r2 from register r1 and store the result in acc
#[inline]
pub fn subr(cpu: &mut CPU) {
    instr!(cpu, rr, wrapping_sub);
}

/// ## MULT 0x1234, r1
/// Multiply register r1 by 0x1234 and store the result in acc
#[inline]
pub fn mult(cpu: &mut CPU) {
    instr!(cpu, wr, wrapping_mul);
}

/// ## MULTR r1, r2
/// Multiply register r2 by register r1 and store the result in acc
#[inline]
pub fn multr(cpu: &mut CPU) {
    instr!(cpu, rr, wrapping_mul);
}

/// ## DIV r1, 0x1234
/// Divide register r1 by 0x1234 and store the result in acc
/// #### Panics if the divisor is 0
#[inline]
pub fn div(cpu: &mut CPU) {
    instr!(cpu, rw, wrapping_div);
}

/// ## DIVWR 0x1234, r1
/// Divide 0x1234 by register r1 and store the result in acc
/// #### Panics if the divisor is 0
#[inline]
pub fn divwr(cpu: &mut CPU) {
    instr!(cpu, wr, wrapping_div);
}

/// ## DIVR r1, r2
/// Divide register r2 by register r1 and store the result in acc
/// #### Panics if the divisor is 0
#[inline]
pub fn divr(cpu: &mut CPU) {
    instr!(cpu, rr, wrapping_div);
}

/// ## INC r1
/// Increment register r1 and store the result in acc
#[inline]
pub fn inc(cpu: &mut CPU) {
	instr!(cpu, cc, wrapping_add);
}

/// ## DEC r1
/// Decrement register r1 and store the result in acc
#[inline]
pub fn dec(cpu: &mut CPU) {
    instr!(cpu, cc, wrapping_sub);
}
