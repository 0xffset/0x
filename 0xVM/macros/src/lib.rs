#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, LitStr, Token, parse::Parser};

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

lazy_static::lazy_static!{
    // Internal, compile-time record of registers.
    static ref REGISTERS: Arc<Mutex<HashMap<String, u32>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[proc_macro]
pub fn init_registers(input: TokenStream) -> TokenStream {
    // Parse input tokens as a comma separated list of string literals.
    let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
    let test = parser.parse(input).unwrap();

    // Form a list from the parsed register list.
    // Use a vec of a tuple here, to preserve the order,
    // Useful for extracting down below.
    let map: Vec<(String, u32)> = test.iter().enumerate()
        .map(|(i, n)| (n.value(), i as u32 * 4))
        .collect();

    // Extract these values to write as output tokens.
    let count = map.len();
    let names: Vec<String> = map.iter().map(|(s, _a)| s.clone()).collect();
    let addresses: Vec<u32> = map.iter().map(|(_s, a)| *a).collect();

    // Update the internal record, by collecting to a hashmap.
    *REGISTERS.lock().unwrap() = map.into_iter().collect();

    (quote!{
        // Slice of register names and their corresponding addresses.
        pub const REGISTERS: &'static [(&'static str, u32)] = &[
            #(
                (#names, #addresses),
            )*
        ];

        // Amount of registers defined.
        pub const REGISTER_COUNT: usize = #count;
    }).into()
}

#[proc_macro]
pub fn reg(input: TokenStream) -> TokenStream {
    // Parse a single string literal as input.
    let name: LitStr = syn::parse(input).unwrap();

    let lock = REGISTERS.lock().unwrap();

    if lock.is_empty() {
        // Drop the lock to ensure no poisoning.
        drop(lock);

        panic!("Registers have not been defined.");
    }

    let address = match lock.get(&name.value()) {
        Some(&a) => a,
        None => {
            // Drop the lock to ensure no poisoning.
            drop(lock);

            panic!("Register '{}' is not a defined register.", name.value());
        }
    };

    (quote! {
        #address
    }).into()
}