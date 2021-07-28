use crate::cpu::CPU;

/// ## POP r1
/// Pop val from stack into register r1
#[inline]
#[allow(non_snake_case)]
pub fn POP(cpu: &mut CPU) {
    let r_addr = cpu.fetch_word();
    let val = cpu.pop();
    cpu.set_reg(r_addr, val);
}

/// ## PUSH 0x1234 
/// Push 0x1234 onto the stack
#[inline]
#[allow(non_snake_case)]
pub fn PUSH(cpu: &mut CPU) {
    let val = cpu.fetch_word();

    cpu.push(val);
}

/// ## PUSHR r1 
/// Push register r1 onto stack
#[inline]
#[allow(non_snake_case)]
pub fn PUSHR(cpu: &mut CPU) {
	let r_addr = cpu.fetch_word();
	let val = cpu.get_reg(r_addr);

	cpu.push(val);
}