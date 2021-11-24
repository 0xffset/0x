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
use std::{env, panic};

mod cpu;
use cpu::{generate_config, VM};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!(
            "[VM] Usage: {0} <config file>\nExample: {0} vm.cfg",
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

    VM::new(&generate_config(args.get(1).unwrap())).run();
}
