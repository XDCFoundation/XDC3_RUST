use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::Address;

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]

/// This method returns Token Name.
/// Token address is required as argument.

pub async fn token_name(token_addr: String) -> web3::Result<String> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_addr).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc20_abi.json"),
    )
    .unwrap();

    let tokenn_name: String = token_contract
        .query("name", (), None, Options::default(), None)
        .await
        .unwrap();

    Ok(tokenn_name)
}
