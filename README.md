# xdc3_rust

xdc3_rust SDK with support for smart contracts and XRC20.

## Usage

Add the following dependency to your `Cargo.toml`:
```toml
[dependencies]
xdc3_rust = "0.1.3"
```
This SDK supports following Read & Write operations in XRC20:-
``1.getName 
  2.getSymbol 
  3.getDecimal 
  4.getTotalSupply 
  5.getBalanceOf 
  6.getAllowance 
  7.getApprove 
  8.increaseAllowance 
  9.decreaseAllowance 
  10.transferfrom 
  11.transferToken 
  12.transferXdc``

## Environment Variable

Create a .env file in the root directory of the Rust project to put the wallet and endpoint information in like so:
```APOTHEM_ADDRESS = wss://ws.apothem.network```

## Example

    #![allow(unused_imports)]

    use std::io::stdin;

    mod lib;
    use lib::xrc20::name;

    #[allow(unused_must_use)]
    fn main() {

       let mut token_address = String::new();
       println!("enter token address: ");
       stdin().read_line(&mut token_address);
    
       let token_name = name::token_name(token_address).unwrap();
       println!("token name: {}", token_name);
    }

This example returns name of the specified address.

## Transports
- [x] WebSockets transport

## Types
- [x] Types for `U256,H256,Address(H160)`
- [x] Transaction type (`Transaction` from Parity)
