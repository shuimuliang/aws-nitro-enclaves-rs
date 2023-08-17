use byteorder::{ByteOrder, LittleEndian};
use serde_json::{json, Map, Value};
use std::io::{Read, Write};
use std::mem::size_of;
use vsock::VsockStream;
use reqwest::Error;
use std::collections::HashMap;

pub fn build_payload(method_name: &str, credential: Map<String, Value>, user_id: String, key_id: String) -> String {
    let mut payload: Map<String, Value> = Map::new();

    payload.insert("apiRequest".to_string(), json!(method_name));
    payload.insert("credential".to_string(), json!(credential));
    payload.insert("name".to_string(), json!(user_id));
    payload.insert("key_id".to_string(), json!(key_id));

    json!(payload).to_string()
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

pub async fn get_iam_token() -> Result<HashMap<String, String>, Error> {
    let client = reqwest::Client::new();
    let instance_profile_name = client.get("http://169.254.169.254/latest/meta-data/iam/security-credentials/")
        .send()
        .await?
        .text()
        .await?;

    let url = format!("http://169.254.169.254/latest/meta-data/iam/security-credentials/{}", instance_profile_name);
    let response: Value = client.get(&url)
        .send()
        .await?
        .json()
        .await?;

    let mut credential = HashMap::new();
    credential.insert("aws_access_key_id".to_string(), response["AccessKeyId"].as_str().unwrap().to_string());
    credential.insert("aws_secret_access_key".to_string(), response["SecretAccessKey"].as_str().unwrap().to_string());
    credential.insert("aws_session_token".to_string(), response["Token"].as_str().unwrap().to_string());

    Ok(credential)
}