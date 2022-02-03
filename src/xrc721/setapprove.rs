use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, TransactionParameters, H256};

#[allow(unused_must_use)]
#[tokio::main]
#[allow(dead_code)]

/// Safely transfers the ownership of a given token ID to another address, If the target address is a contract.
/*
    This function required arguments.
    owner_address, spender_address, token_address, bool_value.
*/

pub async fn set_approval_forall(
    owner_address: String,
    private_key: String,
    spender_address: String,
    token_address: String,
    bool_value: bool,
) -> web3::Result<H256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let owner_addr = Address::from_str(&owner_address).unwrap();

    let bytes_key: &[u8] = &private_key.as_bytes();

    let owner_private_key =
        SecretKey::from_slice(&hex::decode(&bytes_key[0..64]).unwrap()).unwrap();

    let spender_addr = Address::from_str(&spender_address).unwrap();

    let contract_addr = Address::from_str(&token_address).unwrap();

    let boolean_value: bool = bool_value;

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc721_abi.json"),
    )
    .unwrap();

    let out_gas_estimate = token_contract
        .estimate_gas(
            "setApprovalForAll",
            (spender_addr, boolean_value),
            owner_addr,
            Options::default(),
        )
        .await
        .expect("error");

    let gas_price = web3s.eth().gas_price().await.unwrap();

    let data = token_contract
        .abi()
        .function("setApprovalForAll")
        .unwrap()
        .encode_input(&(spender_addr, boolean_value).into_tokens())
        .unwrap();

    let nonce = web3s
        .eth()
        .transaction_count(owner_addr, None)
        .await
        .unwrap();

    let transact_obj = TransactionParameters {
        nonce: Some(nonce),
        to: Some(contract_addr),
        gas_price: Some(gas_price),
        gas: out_gas_estimate,
        data: Bytes(data),
        ..Default::default()
    };

    let signed_transaction = web3s
        .accounts()
        .sign_transaction(transact_obj, &owner_private_key)
        .await
        .unwrap();

    let result: H256 = web3s
        .eth()
        .send_raw_transaction(signed_transaction.raw_transaction)
        .await
        .unwrap();

    Ok(result)
}
