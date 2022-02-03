#![allow(unused_imports)]
use std::io::stdin;

/// Impoting xrc20 & xrc721 module which consits methods.
mod lib;
use lib::xrc20::balanceof::balanceof;
use lib::xrc721::name::token_name;
use lib::xrc20::xdc_account;

#[allow(unused_must_use)]
fn main() {
    // xdc_account::create_account();
   // Taking input as a string from user.
    let mut token_address = String::new();
    println!("enter token address: ");
    stdin().read_line(&mut token_address);

    let mut owner_address = String::new();
    println!("enter token address: ");
    stdin().read_line(&mut owner_address);

    let a = balanceof(token_address, owner_address);

    // let token_name = name::token_name(token_address).unwrap();
    println!("token name: {}", a.unwrap());
}
