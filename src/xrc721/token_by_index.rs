use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[allow(dead_code)]
#[tokio::main]

/// Enumerate NFTs assigned to an owner.
/// tokenAddress An address for whom to query.
/// IndexNO A counter less than `totalSupply()`.
/// The token identifier for the `index`th NFT assigned to `owner`.

pub async fn token_by_index(token_address: String, token_index: i32) -> web3::Result<U256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_address).unwrap();

    let _token_index = (U256::exp10(18) / U256::exp10(18)) * token_index;

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let token_by_index: U256 = token_contract
        .query("tokenByIndex", _token_index, None, Options::default(), None)
        .await
        .unwrap();

    Ok(token_by_index)
}
