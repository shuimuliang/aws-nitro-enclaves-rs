use byteorder::{ByteOrder, LittleEndian};
use clap::Parser;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use vsock::{VsockAddr, VsockListener, VsockStream};

fn handle_client(mut stream: VsockStream) {
    // Buffer to hold the size of the incoming data
    let mut size_buf = [0; size_of::<u64>()];
    stream.read_exact(&mut size_buf).unwrap();

    // Convert the size buffer to u64
    let size = LittleEndian::read_u64(&size_buf);

    // Create a buffer of the size we just read
    let mut buffer = vec![0; size as usize];
    stream.read_exact(&mut buffer).unwrap();

    println!(
        "{}",
        String::from_utf8(buffer.to_vec())
            .map_err(|err| format!("The received bytes are not UTF-8: {:?}", err))?
    );
}

#[derive(Debug, Parser)]
struct Opt {
    /// The encryption key.
    #[structopt(short, long)]
    port: u32,
}

fn main() {
    let Opt { port } = Opt::parse();

    let listener = VsockListener::bind(&VsockAddr::new(libc::VMADDR_CID_ANY, port))
        .expect("bind and listen failed");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream);
    }
}
