use crate::cpu::VM;

/// ## MOVR 0x1234, r1
/// Move 0x1234 into register r1
#[inline]
#[allow(non_snake_case)]
pub fn MOVR(cpu: &mut VM) {
    let val = cpu.fetch_word();
    let r_addr = cpu.fetch_word();
    cpu.set_reg(r_addr, val);
}

/// ## MOVM 0x1234, 0xAF
/// Move 0x1234 into memory at 0xAF
#[inline]
#[allow(non_snake_case)]
pub fn MOVM(cpu: &mut VM) {
    let val = cpu.fetch_word();
    let m_addr = cpu.fetch_word();
    cpu.memory_mapper.set_word(m_addr, val);
}

/// ## MOVRR r1, r2
/// Move register r1 into register r2
#[inline]
#[allow(non_snake_case)]
pub fn MOVRR(cpu: &mut VM) {
    let r1_addr = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    cpu.set_reg(r2_addr, cpu.get_reg(r1_addr));
}

/// ## MOVRM r1, 0xAF
/// Move register r1 into memory ar 0xAF
#[inline]
#[allow(non_snake_case)]
pub fn MOVRM(cpu: &mut VM) {
    let r_addr = cpu.fetch_word();
    let m_addr = cpu.fetch_word();
    cpu.memory_mapper.set_word(m_addr, cpu.get_reg(r_addr));
}

/// ## MOVMR 0xAF, r1
/// Move memory at 0xAF into register r1
#[inline]
#[allow(non_snake_case)]
pub fn MOVMR(cpu: &mut VM) {
    let m_addr = cpu.fetch_word();
    let r_addr = cpu.fetch_word();
    cpu.set_reg(r_addr, cpu.memory_mapper.get_word(m_addr));
}

/// ## MOVRPR r1, r2
/// Move data pointed at by register r1 into register r2
#[inline]
#[allow(non_snake_case)]
pub fn MOVRPR(cpu: &mut VM) {
    let r1_addr = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    let data_addr = cpu.get_reg(r1_addr);

    cpu.set_reg(r2_addr, cpu.memory_mapper.get_word(data_addr));
}

/// ## MOVROR r1, 0x2, r2
/// Move data pointed at by register r1 plus an offset 0x2 into register r2
#[inline]
#[allow(non_snake_case)]
pub fn MOVROR(cpu: &mut VM) {
    let r1_addr = cpu.fetch_word();
    let offset = cpu.fetch_word();
    let r2_addr = cpu.fetch_word();
    let data_addr = cpu.get_reg(r1_addr) + offset;

    cpu.set_reg(r2_addr, cpu.memory_mapper.get_word(data_addr));
}

macro_rules! instr {
    (l, $cpu:ident) => {{
        let addr_ptr = $cpu.fetch_word();
        let addr = $cpu.get_reg(addr_ptr);

        (addr, instr!(size, $cpu))
    }};

    (s, $cpu:ident) => {{
        let size = instr!(size, $cpu);
        let dest_ptr = $cpu.fetch_word();
        let dest = $cpu.get_reg(dest_ptr);

        (size, dest)
    }};

    (op, $cpu:ident, $addr:ident, $size:ident, $dest:ident) => {
        let temp = $cpu.memory_mapper.get_range($addr, $size);
        $cpu.memory_mapper.set_range($dest, temp);
    };

    (size, $cpu:ident) => {{
        let size_reg = $cpu.fetch_word();
        $cpu.get_reg(size_reg)
    }};
}

/// ## LOAD R1, R2, 0x1238
/// Load R2 bytes from device at R1* to memory at 0x1238-0x1238 + R2
#[inline]
#[allow(non_snake_case)]
pub fn LOAD(cpu: &mut VM) {
    let (addr, size) = instr!(l, cpu);
    let dest = cpu.fetch_word();

    instr!(op, cpu, addr, size, dest);
    /*
    let addr_ptr = cpu.fetch_word();
    let addr = cpu.get_reg(addr_ptr);
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest = cpu.fetch_word();

    let temp = cpu.memory_mapper.get_range(addr, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}

/// ## LOADR R1, R2, R3
/// Load R2 bytes from device at R1* to memory at R3*-R3* + R2
#[inline]
#[allow(non_snake_case)]
pub fn LOADR(cpu: &mut VM) {
    let (addr, size) = instr!(l, cpu);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.get_reg(dest_ptr);

    instr!(op, cpu, addr, size, dest);
    /*
    let addr_ptr = cpu.fetch_word();
    let addr = cpu.get_reg(addr_ptr);
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.get_reg(dest_ptr);

    let temp = cpu.memory_mapper.get_range(addr, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}

/// ## LOADM R1, R2, 0x1238
/// Load R2 bytes from device at R1* to memory at 0x1238*-0x1238* + R2
#[inline]
#[allow(non_snake_case)]
pub fn LOADM(cpu: &mut VM) {
    let (addr, size) = instr!(l, cpu);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.memory_mapper.get_word(dest_ptr);

    instr!(op, cpu, addr, size, dest);
    /*
    let addr_ptr = cpu.fetch_word();
    let addr = cpu.get_reg(addr_ptr);
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.memory_mapper.get_word(dest_ptr);

    let temp = cpu.memory_mapper.get_range(addr, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}

/// ## STORE 0x1238, R2, R1
/// Store R2 bytes from memory at 0x1238-0x1238 + R2 to device at R1*
#[inline]
#[allow(non_snake_case)]
pub fn STORE(cpu: &mut VM) {
    let src = cpu.fetch_word();
    let (size, dest) = instr!(s, cpu);

    instr!(op, cpu, src, size, dest);
    /*
    let src = cpu.fetch_word();
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.get_reg(dest_ptr);

    let temp = cpu.memory_mapper.get_range(src, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}

/// ## STORER R3, R2, R1
/// Store R2 bytes from memory at R3*-R3* + R2 to device at R1*
#[inline]
#[allow(non_snake_case)]
pub fn STORER(cpu: &mut VM) {
    let src_ptr = cpu.fetch_word();
    let src = cpu.get_reg(src_ptr);
    let (size, dest) = instr!(s, cpu);

    instr!(op, cpu, src, size, dest);
    /*
    let src_ptr = cpu.fetch_word();
    let src = cpu.get_reg(src_ptr);
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.get_reg(dest_ptr);

    let temp = cpu.memory_mapper.get_range(src, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}

/// ## STOREM 0x1238, R2, R1
/// Store R2 bytes from memory at 0x1238*-0x1238* + R2 to device at R1*
#[inline]
#[allow(non_snake_case)]
pub fn STOREM(cpu: &mut VM) {
    let src_ptr = cpu.fetch_word();
    let src = cpu.memory_mapper.get_word(src_ptr);
    let (size, dest) = instr!(s, cpu);

    instr!(op, cpu, src, size, dest);
    /*
    let src_ptr = cpu.fetch_word();
    let src = cpu.memory_mapper.get_word(src_ptr);
    let size_reg = cpu.fetch_word();
    let size = cpu.get_reg(size_reg);
    let dest_ptr = cpu.fetch_word();
    let dest = cpu.get_reg(dest_ptr);

    let temp = cpu.memory_mapper.get_range(src, size);
    cpu.memory_mapper.set_range(dest, temp);
    */
}
