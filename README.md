# xdc3_rust

xdc3_rust SDK with support for smart contracts and XRC20.

## Usage

First, add this to your `Cargo.toml`:
```toml
[dependencies]
xdc3_rust = "0.1.1"
```
before Running The File enable cargo Extension in your file. EX: run Command -----(cargo init) in Command Line Terminal after That Copy The Path And Goto The File.

Available Operations in XRC20:- 1.getName 2.getSymbol 3.getDecimal 4.getTotalSupply 5.getBalanceOf 6.getAllowance 7.getApprove 8.increaseAllowance 9.decreaseAllowance 10.transferfrom 11.transferToken 12.transferXdc

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



## Transports
- [x] HTTP transport
- [x] IPC transport
- [x] WebSockets transport

## Types
- [x] Types for `U256,H256,Address(H160)`
- [x] Index type (numeric, encoded to hex)
- [x] Transaction type (`Transaction` from Parity)


# Cargo Features

The library supports following features:
- `http` - Enables HTTP transport (requires `tokio` runtime, because of `hyper`).
- `http-tls` - Enables TLS support via `reqwest/default-tls` for HTTP transport (implies `http`; default).
- `http-native-tls` - Enables TLS support via `reqwest/native-tls` for HTTP transport (implies `http`).
- `http-rustls-tls` - Enables TLS support via `reqwest/rustls-tls` for HTTP transport (implies `http`).
- `ws-tokio` - Enables WS transport using `tokio` runtime.
- `ws-tls-tokio` - Enables TLS support for WS transport (implies `ws-tokio`; default).
- `ws-async-std` - Enables WS transport using `async-std` runtime.
- `ws-tls-async-std` - Enables TLS support for WS transport (implies `ws-async-std`).
- `ipc-tokio` - Enables IPC transport using `tokio` runtime (default).
- `signing` - Enable account namespace and local-signing support (default).
- `eip-1193` - Enable EIP-1193 support.
- `arbitrary_precision` - Enable `arbitrary_precision` in `serde_json`.
