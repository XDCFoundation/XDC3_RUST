# xdc3_rust

xdc3_rust SDK with support for smart contracts, XRC20 & XRC721.

## Usage

Add the following dependency to your `Cargo.toml`:
```toml
[dependencies]
xdc3_rust = "0.1.4"
```


## This SDK supports following Read & Write operations:-
```
  |    XRC20 Token: Read methods                    |   XRC20 Token: Write methods                          |
  |     ---                                         |   ---                                                 | 
  |     name()                                      |   approve(receiverAddress , amount)                   |
  |     symbol()                                    |   transfer(recipient, amount)                         |
  |     decimal()                                   |   transferFrom(sender, recipient, amount)             |
  |     totalSupply()                               |   increaseAllowance(spender, addedValue)              |
  |     balanceOf(account)                          |   decreaseAllowance(spender, subtractedValue)         |
  |     allowance(owner, spender)                   |                                                       |
  |                                                 |                                                       |
                                            
  |    XRC721 Token: Read methods                   |   XRC721 Token: Write methods                         |
  |     ----                                        |   ----                                                |
  |     name()                                      |   setApprovalForAll(operatorAddress, booleanValue)    |
  |     symbol()                                    |   approve(receiverAddress , tokenId)                  |
  |     totalSupply()                               |   transferFrom(recipient, tokenId)                    |
  |     balanceOf(owner address)                    |   safeTransferFrom(spender, tokenId)                  |
  |     ownerOf(tokenId)                            |                                                       |
  |     tokenURI(tokenId)                           |                                                       |
  |     tokenByIndex(index)                         |                                                       |
  |     tokenOfOwnerByIndex(ownerAddress,index)     |                                                       |
  |     supportInterface(interfaceId)               |                                                       |
  |     getApproved(tokenId)                        |                                                       |
  |     isApprovedForAll(ownerAddress,operator)     |                                                       |
  |                                                 |                                                       |
             
```

## Environment Variable

Create a .env file in the root directory of the Rust project to put the wallet and endpoint information in like so:
```APOTHEM_ADDRESS = wss://ws.apothem.network```

## Example for XRC20

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

## Example for XRC721

    #![allow(unused_imports)]

    use std::io::stdin;

    mod lib;
    use lib::xrc721::name::token_name;

    #[allow(unused_must_use)]
    fn main() {

       let mut token_address = String::new();
       println!("enter token address: ");
       stdin().read_line(&mut token_address);
    
       let token_name = name::token_name(token_address).unwrap();
       println!("token name: {}", token_name);
    }


## Transports
- [x] WebSockets transport

## Types
- [x] Types for `U256,H256,Address(H160)`
- [x] Transaction type (`Transaction` from Parity)
