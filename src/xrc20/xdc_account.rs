use hdwallet::ExtendedPrivKey;
use hdwallet::KeyIndex;
use rand;

///  This method will print a private key to import account in xdc wallet. 

#[allow(dead_code)]
pub fn create_account() {
    let mut rng = rand::thread_rng();
    let master_key = ExtendedPrivKey::random(&mut rng).unwrap();
    let normal_key_index = KeyIndex::Normal(0);
    let noamal_child_priv_key = master_key.derive_private_key(normal_key_index).unwrap();
    println!("{:?}", &noamal_child_priv_key);
}
