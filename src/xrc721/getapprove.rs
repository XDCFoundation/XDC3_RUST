use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]

/// The approved address for a token ID, or zero if no address set Reverts if the token ID does not exist.
/*
    This function required arguments.
    owner_address, private_key, spender_address, token_addr, amount.
*/

pub async fn get_approve(token_address: String, token_id: i32) -> web3::Result<Address> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_address).unwrap();

    let _token_id = (U256::exp10(18) * token_id) / U256::exp10(18);

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let get_approve: Address = token_contract
        .query("getApproved", _token_id, None, Options::default(), None)
        .await
        .unwrap();

    Ok(get_approve)
}
