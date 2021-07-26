use macros::reg;

use crate::cpu::CPU;

/// ## ADD 0x1234, r1 
/// Add 0x1234 to register r1 and store the result in acc
#[inline(always)]
pub fn add(cpu: &mut CPU) {
	let value = cpu.fetch_word();
	let register_address = cpu.fetch_word();
	let register_value = cpu.get_register(register_address);

	let acc = value.wrapping_add(register_value);

	cpu.set_register(reg!("acc"), acc);

	cpu.update_status_register(value, acc);
}

/// ## ADDR r1, r2
/// Add register r1 and register r2 and store the result in acc
#[inline(always)]
pub fn addr(cpu: &mut CPU) {
	let register1_address = cpu.fetch_word();
	let register2_address = cpu.fetch_word();

	let register1_value = cpu.get_register(register1_address);
	let register2_value = cpu.get_register(register2_address);

	let acc = register1_value.wrapping_add(register2_value);

	cpu.set_register(reg!("acc"), acc);

	cpu.update_status_register(register1_value, acc);
}