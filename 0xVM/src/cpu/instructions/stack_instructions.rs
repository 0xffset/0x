use crate::cpu::CPU;

/// ## POP r1
/// Pop value from stack into register r1
#[inline(always)]
pub fn pop(cpu: &mut CPU) {
    let register_address = cpu.fetch_word();
    let value = cpu.pop();
    cpu.set_register(register_address, value);
}

/// ## PUSH 0x1234 
/// Push 0x1234 onto the stack
#[inline(always)]
pub fn push(cpu: &mut CPU) {
    let value = cpu.fetch_word();

    cpu.push(value);
}

/// ## PUSHR r1 
/// Push register r1 onto stack
#[inline(always)]
pub fn pushr(cpu: &mut CPU) {
	let register_address = cpu.fetch_word();
	let value = cpu.get_register(register_address);

	cpu.push(value);
}