use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[allow(dead_code)]
#[tokio::main]

/// Returns whether the given spender can transfer a given token ID.

pub async fn ownerof(token_address: String, token_id: i32) -> web3::Result<Address> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let _token_id = (U256::exp10(18) * token_id) / U256::exp10(18);

    let contract_addr = Address::from_str(&token_address).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let ownerof: Address = token_contract
        .query("ownerOf", _token_id, None, Options::default(), None)
        .await
        .unwrap();

    Ok(ownerof)
}
