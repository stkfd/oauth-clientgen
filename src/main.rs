use ring::rand::{SystemRandom, SecureRandom};
use ring::digest::SHA512;
use base64::encode as encode_b64;
use hex::encode as encode_hex;

fn main() {
    let rng: SystemRandom = ring::rand::SystemRandom::new();
    let mut client_id = [0u8; 16];
    let mut secret = [0u8; 32];
    rng.fill(&mut client_id).unwrap();
    rng.fill(&mut secret).unwrap();
    let secret_b64 = encode_b64(&secret);
    let secret_hash = ring::digest::digest(&SHA512, secret_b64.as_ref());
    let secret_hash_b64 = encode_b64(&secret_hash);

    println!("Client Id: {}", encode_hex(&client_id));
    println!("Client Secret: {}", secret_b64);
    println!("Client Secret Hash (base64): {}", secret_hash_b64);
}
