use crate::cpu::CPU;

#[allow(non_snake_case)]

/// ## POP r1
/// Pop val from stack into register r1
#[inline]
pub fn POP(cpu: &mut CPU) {
    let r_addr = cpu.fetch_word();
    let val = cpu.pop();
    cpu.set_reg(r_addr, val);
}

/// ## PUSH 0x1234 
/// Push 0x1234 onto the stack
#[inline]
pub fn PUSH(cpu: &mut CPU) {
    let val = cpu.fetch_word();

    cpu.push(val);
}

/// ## PUSHR r1 
/// Push register r1 onto stack
#[inline]
pub fn PUSHR(cpu: &mut CPU) {
	let r_addr = cpu.fetch_word();
	let val = cpu.get_reg(r_addr);

	cpu.push(val);
}