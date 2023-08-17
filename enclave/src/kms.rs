use std::process::Command;
use std::env;
use serde_json::{Map, Value};

pub fn call_kms_generate_datakey(credential: &Map<String, Value>, key_id: &str) -> String {
    let aws_access_key_id = credential["aws_access_key_id"].as_str().unwrap();
    let aws_secret_access_key = credential["aws_secret_access_key"].as_str().unwrap();
    let aws_session_token = credential["aws_session_token"].as_str().unwrap();

    let output = Command::new("/myip/kmstool_enclave_cli")
        .args(&[
            "genkey",
            "--region", &env::var("REGION").unwrap(),
            "--proxy-port", "8000",
            "--aws-access-key-id", &aws_access_key_id,
            "--aws-secret-access-key", &aws_secret_access_key,
            "--aws-session-token", &aws_session_token,
            "--key-id", key_id,
            "--key-spec","AES-256",
        ])
        .output()
        .expect("Failed to execute command");

    let datakey_text = String::from_utf8_lossy(&output.stdout).to_string();
    datakey_text
}