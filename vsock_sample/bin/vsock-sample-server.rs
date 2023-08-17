use clap::Parser;
use serde_json::{Map, Value};
use vsock::{VsockAddr, VsockListener, VsockStream};
use vsock_sample::{build_response, recv_message, send_message};

fn handle_client(mut stream: VsockStream) -> Result<(), anyhow::Error> {
    let payload_buffer = recv_message(&mut stream).map_err(|err| anyhow::anyhow!("{:?}", err))?;

    // Decode the payload as JSON
    let json: Value =
        serde_json::from_slice(&payload_buffer).map_err(|err| anyhow::anyhow!("{:?}", err))?;
    println!("{}", json);

    let content: Map<String, Value> = Map::new();
    let response = build_response("generateResponse", content);
    send_message(&mut stream, response)?;

    Ok(())
}

#[derive(Debug, Parser)]
struct Opt {
    /// server virtio port
    #[structopt(short, long)]
    port: u32,
}

fn main() -> Result<(), anyhow::Error> {
    let Opt { port } = Opt::parse();

    let listener = VsockListener::bind(&VsockAddr::new(libc::VMADDR_CID_ANY, port))
        .expect("bind and listen failed");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let _ = handle_client(stream);
    }

    Ok(())
}
