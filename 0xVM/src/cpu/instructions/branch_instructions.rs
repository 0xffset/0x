use macros::reg;

use crate::cpu::CPU;

/// ## BRBS FLAG_Z, 0xAF
/// If the flag Z is set, jump to 0xAF
#[inline(always)]
pub fn brbs(cpu: &mut CPU) {
    let flag = cpu.fetch_byte();
    let address = cpu.fetch_word();
    if cpu.get_status_flag(flag) {
        cpu.set_register(reg!("pc"), address);
    }
}

/// ## BRBC FLAG_Z, 0xAF
/// If the flag Z is clear, jump to 0xAF
#[inline(always)]
pub fn brbc(cpu: &mut CPU) {
    let flag = cpu.fetch_byte();
    let address = cpu.fetch_word();
    if !cpu.get_status_flag(flag) {
        cpu.set_register(reg!("pc"), address);
    }
}

/// ## BREQ 0x1234, 0x5
/// Jump to 0x5 if acc does equal 0x1234
#[inline(always)]
pub fn breq(cpu: &mut CPU) {
    let value = cpu.fetch_word();
    let address = cpu.fetch_word();

    if cpu.get_register(reg!("acc")) == value {
        cpu.set_register(reg!("pc"), address);
    }
}

/// ## BRNQ 0x1234, 0x5
/// Jump to 0x5 if acc does not equal 0x1234
#[inline(always)]
pub fn brnq(cpu: &mut CPU) {
    let value = cpu.fetch_word();
    let address = cpu.fetch_word();

    if cpu.get_register(reg!("acc")) != value {
        cpu.set_register(reg!("pc"), address);
    }
}
