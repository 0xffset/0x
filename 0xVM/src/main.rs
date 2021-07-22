#![feature(panic_info_message)]

use macros::init_registers;

init_registers![
    "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8", // general purpose registers
    "pc", // program counter
    "acc", // accumulator
    "sr", // status register
    "sp", // stack pointer
    "fp", // frame pointer
];

mod device;

mod memory;
use std::{env, fs::File, io::Read, panic};

use memory::{Byte, Memory, Word};
mod cpu;
use cpu::CPU;

use crate::{device::Device, memory::MemoryMapper};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("[VM] Usage: {0} <program_path>\nExample: {0} a.bin", args.get(0).unwrap());
    }
    
    // custom panic outputs
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.message() {
            println!("0xVM panicked at\n{}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("0xVM panicked at\n{}", s);
        } else {
            println!("0xVM panicked!");
        }
    }));
    
    let mut bin = match File::open(args.get(1).unwrap()) {
        Ok(file) => file,
        Err(_) => panic!("[VM] Failed to open program file"),
    };
    
    let mut memory = Memory::new(0xFFFF);
    // write program into memory
    let mut buff = Vec::<Byte>::new();
    bin.read_to_end(&mut buff).unwrap();
    for (i, b) in buff.iter().enumerate() {
        memory.set_byte(i as Word, *b);
    }

    let mut MM = MemoryMapper::new();
    MM.map(Box::new(memory), 0, 0xFFFF, true);

    let mut cpu = CPU::new(MM);
    
    //cpu.run();
    // debug
    cpu.run_debug(0xFFFF - 70, 70);
}
