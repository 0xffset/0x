#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, LitStr, LitInt, Token, parse::Parser};

use std::collections::HashMap;
use std::sync::{ Arc, Mutex };

lazy_static::lazy_static!{
    static ref REGISTRIES: Arc<Mutex<HashMap<String, u8>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[proc_macro]
pub fn init_registries(input: TokenStream) -> TokenStream {
    let parser = Punctuated::<LitStr, Token![,]>::parse_terminated;
    let test = parser.parse(input).unwrap();

    let map: HashMap<String, u8> = test.iter().enumerate()
        .map(|(i, n)| (n.value(), i as u8 * 4))
        .collect();

    let count = map.len();

    let names: Vec<String> = map.keys().cloned().collect();
    let addresses: Vec<u8> = map.values().cloned().collect();

    *REGISTRIES.lock().unwrap() = map;

    let count_const = quote!{
        pub const REGISTERS: &'static [(&'static str, u8)] = &[
            #(
                (#names, #addresses),
            )*
        ];

        pub const REGISTER_COUNT: usize = #count;
    };
    
    TokenStream::from(count_const)
}

#[proc_macro]
pub fn reg(input: TokenStream) -> TokenStream {
    let name: LitStr = syn::parse(input).unwrap();

    let lock = REGISTRIES.lock().unwrap();

    if lock.is_empty() {
        drop(lock);

        panic!("Registries have not been defined.");
    }

    let address = match lock.get(&name.value()) {
        Some(&a) => {
            a
        },
        None => {
            drop(lock);

            panic!("Registry '{}' is not a defined registry.", name.value());
        }
    };

    (quote! {
        #address
    }).into()
}