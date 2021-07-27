use crate::cpu::CPU;

/// ## POP r1
/// Pop val from stack into register r1
#[inline(always)]
pub fn pop(cpu: &mut CPU) {
    let r_addr = cpu.fetch_word();
    let val = cpu.pop();
    cpu.set_reg(r_addr, val);
}

/// ## PUSH 0x1234 
/// Push 0x1234 onto the stack
#[inline(always)]
pub fn push(cpu: &mut CPU) {
    let val = cpu.fetch_word();

    cpu.push(val);
}

/// ## PUSHR r1 
/// Push register r1 onto stack
#[inline(always)]
pub fn pushr(cpu: &mut CPU) {
	let r_addr = cpu.fetch_word();
	let val = cpu.get_reg(r_addr);

	cpu.push(val);
}