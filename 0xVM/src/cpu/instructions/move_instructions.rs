use crate::cpu::CPU;

/// ## MOVR 0x1234, r1
/// Move 0x1234 into register r1
#[inline(always)]
pub fn movr(cpu: &mut CPU) {
    let value = cpu.fetch_word();
    let register_address = cpu.fetch_word();
    cpu.set_register(register_address, value);
}

/// ## MOVM 0x1234, 0xAF
/// Move 0x1234 into memory at 0xAF
#[inline(always)]
pub fn movm(cpu: &mut CPU) {
    let value = cpu.fetch_word();
    let memory_address = cpu.fetch_word();
    cpu.memory_mapper.set_word(memory_address, value);
}

/// ## MOVRR r1, r2
/// Move register r1 into register r2
#[inline(always)]
pub fn movrr(cpu: &mut CPU) {
    let register1_address = cpu.fetch_word();
    let register2_address = cpu.fetch_word();
    cpu.set_register(register2_address, cpu.get_register(register1_address));
}

/// ## MOVRM r1, 0xAF
/// Move register r1 into memory ar 0xAF
#[inline(always)]
pub fn movrm(cpu: &mut CPU) {
    let register_address = cpu.fetch_word();
    let memory_address = cpu.fetch_word();
    cpu.memory_mapper
        .set_word(memory_address, cpu.get_register(register_address));
}

/// ## MOVMR 0xAF, r1
/// Move memory at 0xAF into register r1
#[inline(always)]
pub fn movmr(cpu: &mut CPU) {
    let memory_address = cpu.fetch_word();
    let register_address = cpu.fetch_word();
    cpu.set_register(register_address, cpu.memory_mapper.get_word(memory_address));
}

/// ## MOVRPR r1, r2
/// Move data pointed at by register r1 into register r2
#[inline(always)]
pub fn movrpr(cpu: &mut CPU) {
    let register1_address = cpu.fetch_word();
    let register2_address = cpu.fetch_word();
    let data_address = cpu.get_register(register1_address);

    cpu.set_register(register2_address, cpu.memory_mapper.get_word(data_address));
}

/// ## MOVROR r1, 0x2, r2
/// Move data pointed at by register r1 plus an offset 0x2 into register r2
#[inline(always)]
pub fn movror(cpu: &mut CPU) {
    let register1_address = cpu.fetch_word();
    let offset = cpu.fetch_word();
    let register2_address = cpu.fetch_word();
    let data_address = cpu.get_register(register1_address) + offset;

    cpu.set_register(register2_address, cpu.memory_mapper.get_word(data_address));
}
