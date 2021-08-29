#![feature(panic_info_message)]
#![feature(type_name_of_val)]

use macros::init_registers;

init_registers![
    "r1", "r2", "r3", "r4", "r5", "r6", "r7", "r8",  // general purpose registers
    "pc",  // program counter
    "acc", // accumulator
    "sr",  // status register
    "sp",  // stack pointer
    "fp",  // frame pointer
];

mod device;

mod memory;
use std::{env, fs::File, io::Read, panic};

use memory::{Byte, Memory};
mod cpu;
use cpu::CPU;

use crate::{device::Screen, memory::MemoryMapper};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "[VM] Usage: {0} <program_path>\nExample: {0} a.bin",
            args.get(0).unwrap()
        );
    }

    // custom panic outputs
    panic::set_hook(Box::new(|panic_info| {
        if let Some(s) = panic_info.message() {
            println!("0xVM panicked:\n{}", s);
        } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
            println!("0xVM panicked:\n{}", s);
        } else {
            println!("0xVM panicked!");
        }
    }));

    let mut bin = match File::open(args.get(1).unwrap()) {
        Ok(file) => file,
        Err(_) => panic!("[VM] Failed to open program file"),
    };

    // write program into buffer to be coppied into memory
    let mut buff = Vec::<Byte>::new();
    bin.read_to_end(&mut buff).unwrap();

    // #################
    // # PROGRAM START #
    // #################    
    
    let memory = Memory::from(buff, 0xFFFF);
    let screen = Screen::new(16, 16);

    let mut mm = MemoryMapper::new();
    mm.map(Box::new(screen), 0, 0x400);
    mm.map(Box::new(memory), 0x400, 0xFFFF);

    let mut cpu = CPU::new(mm);
    cpu.set_stack(0xFFFF, 1024);

    cpu.run();
}
