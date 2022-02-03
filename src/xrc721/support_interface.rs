use std::env;
use std::str::FromStr;
use web3::contract::{Contract, Options};
use web3::types::Address;

#[allow(unused_must_use)]
#[allow(dead_code)]
#[tokio::main]

/// Query if a contract implements an interface.
/// tokenAddress An address for whom to query and x_bytes The interface identifier.
/// `true` if the contract implements `interfaceID` andinterfaceID` is not 0xffffffff, `false` otherwise.

pub async fn support_interface(token_address: String, interface_id: u32) -> web3::Result<bool> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let contract_addr = Address::from_str(&token_address).unwrap();

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let x_bytes = interface_id.to_be_bytes();

    let support_interface: bool = token_contract
        .query("supportsInterface", x_bytes, None, Options::default(), None)
        .await
        .unwrap();

    Ok(support_interface)
}
