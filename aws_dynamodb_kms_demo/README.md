Provides a list of commands to interact with AWS DynamoDB, KMS using the Rust.

### Commands:

#### Create a DynamoDB table:
```sh
cargo run --bin aws-dynamodb-create-table -- -r ap-east-1 -t AccountTable -k name
```

#### Add an item to the DynamoDB table:
```sh
cargo run --bin aws-dynamodb-add-item -- \
  -r ap-east-1 \
  -t AccountTable \
  -n UID10000 \
  -k 0a5713c9-7d29-4b8e-aa0e-8e27e58ac6e1 \
  -e encrypted_private_key \
  -a 0x1f9090aae28b8a3dceadf281b0f12828e676c326 \
  -d encrypted_private_key
```

#### Get an item from the DynamoDB table:
```sh
cargo run --bin aws-dynamodb-get-item -- -r ap-east-1 -t AccountTable -n UID10000 
cargo run --bin aws-dynamodb-get-item -- -r ap-east-1 -t AccountTable -n UID10001
```

#### Create a KMS key:
```sh
cargo run --bin aws-kms-create-key -- -r ap-east-1
```

#### Encrypt data using the KMS key:
```sh
cargo run --bin aws-kms-encrypt -- -r ap-east-1 -k 0a5713c9-7d29-4b8e-aa0e-8e27e58ac6e1 -o /tmp/kms-encrypt.txt -t KeyId1
```

#### Decrypt data using the KMS key:
```sh
cargo run --bin aws-kms-decrypt -- -r ap-east-1 -k 0a5713c9-7d29-4b8e-aa0e-8e27e58ac6e1 -i /tmp/kms-encrypt.txt
```

#### Generate a data key using the KMS key:
```sh
cargo run --bin aws-kms-generate-data-key -- -r ap-east-1 -k 0a5713c9-7d29-4b8e-aa0e-8e27e58ac6e1
```
