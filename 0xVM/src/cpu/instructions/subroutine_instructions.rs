use macros::reg;

use crate::cpu::CPU;

/// ## CALL 0xAF 
/// Call subroutine at 0xAF
#[inline(always)]
pub fn call(cpu: &mut CPU) {
	let address = cpu.fetch_word();

	cpu.push_state();

	cpu.set_register(reg!("pc"), address);
}

/// ## CALLR r1 
/// Call subroutine at r1
#[inline(always)]
pub fn callr(cpu: &mut CPU) {
	let register_address = cpu.fetch_word();
	let address = cpu.get_register(register_address);

	cpu.push_state();

	cpu.set_register(reg!("pc"), address);
}

/// ## RET 
/// Return from subroutine
#[inline(always)]
pub fn ret(cpu: &mut CPU) {
	cpu.pop_state();
}