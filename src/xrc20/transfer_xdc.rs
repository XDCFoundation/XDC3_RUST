use secp256k1::SecretKey;
use std::env;
use std::str::FromStr;
use web3::types::{Address, TransactionParameters, H256, U256};

#[tokio::main]
#[allow(dead_code)]

/// Transfer XDC for a specified address.
/// This function requires following arguments.
/// private key, recipient address, amount. 

pub async fn transfer_xdc(
    private_key: String,
    recipient_address: String,
    amount: String,
) -> web3::Result<H256> {
    dotenv::dotenv().ok();

    let websocket = web3::transports::WebSocket::new(&env::var("APOTHEM_ADDRESS").unwrap()).await?;
    let web3s = web3::Web3::new(websocket);

    let bytes_key: &[u8] = &private_key.as_bytes();

    let owner_private_key =
        SecretKey::from_slice(&hex::decode(&bytes_key[0..64]).unwrap()).unwrap();
    let recipient_addr = Address::from_str(&recipient_address).unwrap();

    let amount: i32 = amount.trim().parse().expect("invalid input");

    let tx_object = TransactionParameters {
        to: Some(recipient_addr),
        value: U256::exp10(18) * amount,
        ..Default::default()
    };

    let signed = web3s
        .accounts()
        .sign_transaction(tx_object, &owner_private_key)
        .await?;

    let result: H256 = web3s
        .eth()
        .send_raw_transaction(signed.raw_transaction)
        .await?;

    Ok(result)
}
