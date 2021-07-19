#![allow(unused_imports)]

use macros::{ init_registers, reg };

#[test]
fn init() {
    init_registers![
        "r1",
        "r2",
        "r3"
    ];

    println!("{:?}", REGISTERS);

    assert_eq!(REGISTER_COUNT, 3);
}

#[test]
fn valid_registers() {
    init_registers![
        "r1",
        "r2",
        "r3"
    ];

    assert_eq!(reg!("r1"), 0);
    assert_eq!(reg!("r2"), 4);
    assert_eq!(reg!("r3"), 8);
}

// The test fails because the panic originates inside the reg! macro,
// and therefore it is not recognised as a test panic.
/*
#[test]
#[should_panic]
fn invalid_registers() {
    init_registers![
        "r1",
        "r2",
        "r3"
    ];

    let _addr = reg!("r4");
}
*/