use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::{Address, U256};

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]
/// This method returns how much allowance spender have from owner.
/*
    This function required three arguments.
    owner address, spender address, token address. 
*/
pub async fn allowance(
    owner_address: String,
    spender_address: String,
    token_addr: String,
) -> web3::Result<U256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let owner_addr = Address::from_str(&owner_address).unwrap();

    let spender_addr = Address::from_str(&spender_address).unwrap();

    let contract_addr = Address::from_str(&token_addr).unwrap();
    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/erc20_abi.json"),
    )
    .unwrap();

    let allowance: U256 = token_contract
        .query(
            "allowance",
            (owner_addr, spender_addr),
            None,
            Options::default(),
            None,
        )
        .await
        .unwrap();

    let _allowance = allowance / U256::exp10(18);

    Ok(_allowance)
}
