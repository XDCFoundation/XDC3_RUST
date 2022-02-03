use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::contract::tokens::Tokenize;
use web3::contract::{Contract, Options};
use web3::types::{Address, Bytes, TransactionParameters, H256, U256};

#[allow(unused_must_use)]
#[allow(dead_code)]
#[tokio::main]

/// Transfer tokens from one address to another.
/// This function requires following arguments.
/// owner address, private key, spender address, recipient address, token address, amount. 

pub async fn transfer_from(
    owner_address: String,
    spender_address: String,
    private_key: String,
    recipient_address: String,
    token_address: String,
    amount: String,
) -> web3::Result<H256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let owner_addr = Address::from_str(&owner_address).unwrap();

    let spender_addr = Address::from_str(&spender_address).unwrap();

    let bytes_key: &[u8] = &private_key.as_bytes();

    let spender_private_key =
        SecretKey::from_slice(&hex::decode(&bytes_key[0..64]).unwrap()).unwrap();

    let recipient_addr = Address::from_str(&recipient_address).unwrap();

    let contract_addr = Address::from_str(&token_address).unwrap();

    let amount: i32 = amount.trim().parse().expect("invalid input");

    let token_contract = Contract::from_json(
        web3s.eth(),
        contract_addr,
        include_bytes!("../common/xrc20_abi.json"),
    )
    .unwrap();

    let out_gas_estimate = token_contract
        .estimate_gas(
            "transferFrom",
            (owner_addr, recipient_addr, U256::exp10(18) * amount),
            spender_addr,
            Options::default(),
        )
        .await
        .expect("error");

    let gas_price = web3s.eth().gas_price().await.unwrap();

    let data = token_contract
        .abi()
        .function("transferFrom")
        .unwrap()
        .encode_input(&(owner_addr, recipient_addr, U256::exp10(18) * amount).into_tokens())
        .unwrap();

    let nonce = web3s
        .eth()
        .transaction_count(spender_addr, None)
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
        .sign_transaction(transact_obj, &spender_private_key)
        .await
        .unwrap();

    let result: H256 = web3s
        .eth()
        .send_raw_transaction(signed_transaction.raw_transaction)
        .await
        .unwrap();

    Ok(result)
}
