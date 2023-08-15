use clap::Parser;
use std::io::Write;
use vsock::{VsockAddr, VsockStream};
use vsock_sample::{build_payload, send_message};
use serde_json::Map;

#[derive(Debug, Parser)]
struct Opt {
    ///
    #[structopt(short, long)]
    cid: u32,

    /// The port
    #[structopt(short, long)]
    port: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let Opt { cid, port } = Opt::parse();

    // Initiate a connection on an AF_VSOCK socket
    let mut stream = VsockStream::connect(&VsockAddr::new(cid, port)).expect("connection failed");

    // build payload
    let credential = Map::new();
    let key_id = "key_id".to_string();
    let payload = build_payload("generateAccount", credential, key_id);

    // send payload
    send_message(&mut stream, payload)?;

    // recv response

    Ok(())
}
