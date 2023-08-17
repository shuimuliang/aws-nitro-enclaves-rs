use anychain_ethereum::{address::EthereumAddress, public_key::EthereumPublicKey};
use base64::{engine::general_purpose, Engine as _};
use byteorder::{ByteOrder, LittleEndian};
use libsecp256k1::{PublicKey, SecretKey};
use rand::rngs::OsRng;
use serde_json::{json, Map, Value};
use std::io::{Read, Write};
use std::mem::size_of;
use vsock::VsockStream;

pub mod kms;

pub fn generate_random_secret_key() -> String {
    let mut rng = OsRng;

    let secret_key = SecretKey::random(&mut rng);
    let secret_key_bytes = secret_key.serialize();
    let secret_key_base64 = general_purpose::STANDARD.encode(secret_key_bytes);
    println!("Generated secret key: {:?}", secret_key);
    println!("Generated secret key (base64): {:?}", secret_key_base64);

    let public_key = PublicKey::from_secret_key(&secret_key);
    println!("Generated public key: {:?}", public_key);

    let ethereum_public_key = EthereumPublicKey::from_secp256k1_public_key(public_key);
    let ethereum_address = EthereumAddress::checksum_address(&ethereum_public_key);
    println!(
        "Generated Ethereum address: {:?}",
        ethereum_address.to_string()
    );

    secret_key_base64
}

pub fn build_response(method_name: &str, content: Map<String, Value>) -> String {
    let mut payload: Map<String, Value> = Map::new();

    payload.insert("apiResponse".to_string(), json!(method_name));
    payload.insert("content".to_string(), json!(content));

    json!(payload).to_string()
}

pub fn send_message(stream: &mut VsockStream, msg: String) -> Result<(), anyhow::Error> {
    // write message length
    let payload_len: u64 = msg
        .len()
        .try_into()
        .map_err(|err: std::num::TryFromIntError| anyhow::anyhow!("{:?}", err))?;
    let mut header_buf = [0; size_of::<u64>()];
    LittleEndian::write_u64(&mut header_buf, payload_len);
    stream
        .write(&header_buf)
        .map_err(|err| anyhow::anyhow!("{:?}", err))?;

    // write message body
    let payload_buf = msg.as_bytes();
    stream
        .write_all(payload_buf)
        .map_err(|err| anyhow::anyhow!("{:?}", err))?;

    Ok(())
}

pub fn recv_message(stream: &mut VsockStream) -> Result<Vec<u8>, anyhow::Error> {
    // Buffer to hold the size of the incoming data
    let mut size_buf = [0; size_of::<u64>()];
    stream.read_exact(&mut size_buf).unwrap();

    // Convert the size buffer to u64
    let size = LittleEndian::read_u64(&size_buf);

    // Create a buffer of the size we just read
    let mut payload_buffer = vec![0; size as usize];
    stream.read_exact(&mut payload_buffer).unwrap();

    Ok(payload_buffer)
}
