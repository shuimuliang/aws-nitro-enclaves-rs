use clap::Parser;
use parent::{
    iam::get_iam_token,
    protocol_helper::{build_payload, recv_message, send_message},
};
use serde_json::{Map, Value};
use vsock::{VsockAddr, VsockStream};

#[derive(Debug, Parser)]
struct Opt {
    /// CID
    #[structopt(short, long)]
    cid: u32,

    /// The port
    #[structopt(short, long)]
    port: u32,

    /// KMS key id
    #[structopt(short, long)]
    key_id: String,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let Opt { cid, port, key_id } = Opt::parse();

    // Initiate a connection on an AF_VSOCK socket
    let mut stream = VsockStream::connect(&VsockAddr::new(cid, port)).expect("connection failed");

    // build payload
    let credential = get_iam_token().await.unwrap();

    let credential: Map<String, Value> = credential
        .into_iter()
        .map(|(k, v)| (k, Value::String(v)))
        .collect();
    let user_id = "UID10000".to_string();
    let payload = build_payload("generateAccount", credential, user_id, key_id);

    // send payload
    send_message(&mut stream, payload)?;

    // recv response
    let response = recv_message(&mut stream).map_err(|err| anyhow::anyhow!("{:?}", err))?;

    // Decode the payload as JSON
    let json: Value =
        serde_json::from_slice(&response).map_err(|err| anyhow::anyhow!("{:?}", err))?;
    println!("response {}", json);

    Ok(())
}
