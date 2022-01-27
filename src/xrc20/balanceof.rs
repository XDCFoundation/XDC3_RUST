use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[allow(dead_code)]
#[tokio::main]

/// Gets how much allowance spender have from owner.
/// In this method token address, owner address are required as arguments.
pub async fn balanceof(token_addr: String, owner_address: String) -> web3::Result<U256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_addr).unwrap();

    let owner_addr = Address::from_str(&owner_address).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/erc20_abi.json"),
    )
    .unwrap();

    let balanceof: U256 = token_contract
        .query("balanceOf", owner_addr, None, Options::default(), None)
        .await
        .unwrap();

    let _balanceof = balanceof / U256::exp10(18);

    Ok(_balanceof)
}
