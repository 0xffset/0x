use crate::cpu::CPU;

/// ## MOVR 0x1234, r1
/// Move 0x1234 into register r1
#[inline]
pub fn movr(cpu: &mut CPU) {
    let val = cpu.fetch_word();
    let r_addr = cpu.fetch_word();
    cpu.set_reg(r_addr, val);
}

/// ## MOVM 0x1234, 0xAF
/// Move 0x1234 into memory at 0xAF
#[inline]
pub fn movm(cpu: &mut CPU) {
    let val = cpu.fetch_word();
    let m_addr = cpu.fetch_word();
    cpu.memory_mapper.set_word(m_addr, val);
}

/// ## MOVRR r1, r2
/// Move register r1 into register r2
#[inline]
pub fn movrr(cpu: &mut CPU) {
    let r1_addr = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    cpu.set_reg(r2_addr, cpu.get_reg(r1_addr));
}

/// ## MOVRM r1, 0xAF
/// Move register r1 into memory ar 0xAF
#[inline]
pub fn movrm(cpu: &mut CPU) {
    let r_addr = cpu.fetch_word();
    let m_addr = cpu.fetch_word();
    cpu.memory_mapper
        .set_word(m_addr, cpu.get_reg(r_addr));
}

/// ## MOVMR 0xAF, r1
/// Move memory at 0xAF into register r1
#[inline]
pub fn movmr(cpu: &mut CPU) {
    let m_addr = cpu.fetch_word();
    let r_addr = cpu.fetch_word();
    cpu.set_reg(r_addr, cpu.memory_mapper.get_word(m_addr));
}

/// ## MOVRPR r1, r2
/// Move data pointed at by register r1 into register r2
#[inline]
pub fn movrpr(cpu: &mut CPU) {
    let r1_addr = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    let data_addr = cpu.get_reg(r1_addr);

    cpu.set_reg(r2_addr, cpu.memory_mapper.get_word(data_addr));
}

/// ## MOVROR r1, 0x2, r2
/// Move data pointed at by register r1 plus an offset 0x2 into register r2
#[inline]
pub fn movror(cpu: &mut CPU) {
    let r1_addr = cpu.fetch_word();
    let offset = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    let data_addr = cpu.get_reg(r1_addr) + offset;

    cpu.set_reg(r2_addr, cpu.memory_mapper.get_word(data_addr));
}
