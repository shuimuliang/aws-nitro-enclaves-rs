use serde_json::{Map, Value};
use std::env;
use subprocess::{Exec, Redirection};

pub fn call_kms_generate_datakey(credential: &Map<String, Value>, key_id: &str) -> String {
    let aws_access_key_id = credential["aws_access_key_id"].as_str().unwrap();
    let aws_secret_access_key = credential["aws_secret_access_key"].as_str().unwrap();
    let aws_session_token = credential["aws_session_token"].as_str().unwrap();

    let output = Exec::cmd("/myip/kmstool_enclave_cli")
        .args(&[
            "genkey",
            "--region",
            &env::var("REGION").unwrap(),
            "--proxy-port",
            "8000",
            "--aws-access-key-id",
            &aws_access_key_id,
            "--aws-secret-access-key",
            &aws_secret_access_key,
            "--aws-session-token",
            &aws_session_token,
            "--key-id",
            key_id,
            "--key-spec",
            "AES-256",
        ])
        .stdout(Redirection::Pipe)
        .capture()
        .expect("Failed to execute command")
        .stdout_str();

    println!("{}", output);
    output
}
