use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]

/// Gets the Decimal of the specified address.
/// Token address is required as argument.

pub async fn decimals(token_addr: String) -> web3::Result<U256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_addr).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/erc20_abi.json"),
    )
    .unwrap();

    let decimals_method: U256 = token_contract
        .query("decimals", (), None, Options::default(), None)
        .await
        .unwrap();

    Ok(decimals_method)
}
