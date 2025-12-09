use hex;
use rand::RngCore;

pub fn generate_api_key() -> String {
    let mut key = [0u8; 32];
    rand::rng().fill_bytes(&mut key);
    hex::encode(key)
}
