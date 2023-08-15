use byteorder::{ByteOrder, LittleEndian};
use std::io::Write;
use std::mem::size_of;
use vsock::VsockStream;
use serde_json::{json, Map, Value};

pub fn build_payload(method_name: &str, credential: Map<String, Value>, key_id: String) -> String {
    let mut payload: Map<String, Value> = Map::new();

    payload.insert("apiCall".to_string(), json!(method_name));
    payload.insert("credential".to_string(), json!(credential));
    payload.insert("keyId".to_string(), json!(key_id));

    json!(payload).to_string()
}

pub fn send_message(stream: &mut VsockStream, msg: String) -> Result<(), anyhow::Error> {
    // write message length
    let payload_len: u64 = msg
        .len()
        .try_into()
        .map_err(|err: std::num::TryFromIntError| anyhow::anyhow!("{:?}", err))?;
    let mut header_buf = [0u8; size_of::<u64>()];
    LittleEndian::write_u64(&mut header_buf, payload_len);

    // write message body
    let payload_buf = msg.as_bytes();
    stream.write_all(payload_buf).map_err(|err| anyhow::anyhow!("{:?}", err))?;

    Ok(())
}
