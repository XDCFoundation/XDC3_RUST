use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::Address;

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]

/// Tells whether an operator is approved by a given owner.
/*
    This function required arguments.
    owner_address, spender_address, token_addr, amount.
*/

pub async fn is_approve(
    owner_address: String,
    spender_address: String,
    token_address: String,
) -> web3::Result<bool> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let owner_addr = Address::from_str(&owner_address).unwrap();

    let spender_addr = Address::from_str(&spender_address).unwrap();

    let contract_addr = Address::from_str(&token_address).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let is_approve: bool = token_contract
        .query(
            "isApprovedForAll",
            (owner_addr, spender_addr),
            None,
            Options::default(),
            None,
        )
        .await
        .unwrap();

    Ok(is_approve)
}
