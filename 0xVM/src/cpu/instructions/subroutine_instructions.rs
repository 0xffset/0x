use macros::reg;

use crate::cpu::CPU;

/// ## JMP 0xAF
/// Jumps to addr 0xAF
#[inline]
pub fn jmp(cpu: &mut CPU) {
	let addr = cpu.fetch_word();

	cpu.set_reg(reg!("pc"), addr);
}

/// ## CALL 0xAF 
/// Call subroutine at 0xAF
#[inline]
pub fn call(cpu: &mut CPU) {
	let addr = cpu.fetch_word();

	cpu.push_state();

	cpu.set_reg(reg!("pc"), addr);
}

/// ## CALLR r1 
/// Call subroutine at r1
#[inline]
pub fn callr(cpu: &mut CPU) {
	let r_addr = cpu.fetch_word();
	let addr = cpu.get_reg(r_addr);

	cpu.push_state();

	cpu.set_reg(reg!("pc"), addr);
}

/// ## RET 
/// Return from subroutine
#[inline]
pub fn ret(cpu: &mut CPU) {
	cpu.pop_state();
}