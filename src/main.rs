#![allow(unused_imports)]

use std::io::stdin;

mod lib;
/// Impoting name module which consits token_name method.
use lib::xrc20::name;


#[allow(unused_must_use)]
fn main() {
    // Taking input as a string from user.
    let mut token_address = String::new();
    println!("enter token address: ");
    stdin().read_line(&mut token_address);
    
    // Passing user value as function required.
    let token_name = name::token_name(token_address).unwrap();
    println!("token name: {}", token_name);
}