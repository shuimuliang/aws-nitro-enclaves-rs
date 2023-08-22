# aws-nitro-enclaves-rs

## Architecture Diagram
[Encrypted Wallet Application Based on Nitro Enclaves + AWS EKS](https://aws.amazon.com/cn/blogs/china/crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks/)

build a secure and trusted execution environment based on nitro enclave3

![build-a-secure-and-trusted-execution-environment-based-on-nitro-enclave3.png](docs%2Fimages%2Fbuild-a-secure-and-trusted-execution-environment-based-on-nitro-enclave3.png)

## Generate Account
![crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks2.png](docs%2Fimages%2Fcrypto-wallet-application-based-on-nitro-enclaves-and-aws-eks2.png)

### Account Generation Workflow

1. parent instance client receive generateccount API call
2. call getlAMToken function which get credential of IAM Role
3. send credential and generateAccount API call via vsock
4. generate account in Nitro Enclave
5. call KMS API generateDataKey with credential and attestation
6. encrypt the account content with datakey
7. send the encrypted account content to parent instance via vsock
8. call API to save the encrypted account content to dynamodb

## Sign Signature by Private Key
![crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks3.png](docs%2Fimages%2Fcrypto-wallet-application-based-on-nitro-enclaves-and-aws-eks3.png)

### Transaction Signature Workflow

1. parent instance client receive sign API call
2. call getlAMToken function which get credential of IAM Role
3. send credential and sign API call via vsock
4. decrypt the encrypted datakey with KMS API decrypt
5. decrypt the encrypted wallet's private key with datakey
6. sign the message with wallet's private key
7. send signature to parent instance via vsock

## Core Components
### Nitro Enclaves vsock sample client
vsock_sample/bin/vsock-sample-client.rs

How to build:
```sh
cd vsock_sample
make client
```

### Nitro Enclaves vsock sample server
vsock_sample/bin/vsock-sample-server.rs

How to build:
```sh
cd vsock_sample
make server
```

### Socket Protocol helpers
vsock_sample/src/lib.rs

### enclave server
enclave/bin/enclave-server

How to build & run:
```sh
cd enclave
make server
nitro-cli run-enclave --cpu-count 2 --memory 512 --enclave-cid 16 --eif-path enclave-server.eif --debug-mode
```

### parent client
How to build & run:
```sh
cargo build --release
../target/release/enclave-client -c 16 -p 7878 -k <kms key id> -t AccountTable
```

# important configurations
when you try to run the Nitro Enclave application, you should configure below things

- **AWS Region** : you need choose a region where you deploy your application, and you need set **ENV** in Dockerfile
```
ENV REGION ap-east-1
```

- **AWS KMS**
you need create a **Symmetric** kms key, which used for **Encrypt and decrypt**
```sh
cargo run --bin aws-kms-create-key -- -r ap-east-1
```

- **AWS IAM Role**
 
you need create a IAM Role which will be attached to EC2/EKS, it need have the access for kms and dynamodb. you need update this policy after your enclave image created with condition check of PCR
`EnclavePolicyKmsDynamodbTemplate`
```
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": [
                "kms:Decrypt",
                "kms:GenerateDataKey"
            ],
            "Resource": "<Your KMS ARN>"
        },
        {
            "Sid": "VisualEditor1",
            "Effect": "Allow",
            "Action": [
                "dynamodb:PutItem",
                "dynamodb:GetItem"
            ],
            "Resource": "*"
        }
    ]
}
```

`EnclaveRole`, attach policy EnclavePolicyKmsDynamodbTemplate, attach the role on EC2

- **AWS DynamoDB**

 Table Name:  AccountTable 


| Column               | Description                                         |
| -------------------- | --------------------------------------------------- |
| KeyId                | KMS key id used for encryption on the wallet private key |
| Name                 | Account name for this account, used to identify wallet |
| EncryptedPrivateKey  | Encrypted wallet private key                        |
| Address              | The address key of the wallet                       |
| EncryptedDataKey     | The data key used to encrypt the private key        |


- **vsock-proxy** : before you start the enclave application, you should start the vsock-proxy for kms. below command with run the proxy on parent instance which will forward request on port 8000 to endpoint kms.ap-east-1.amazonaws.com on port 443. you should run it before run enclave
```sh
sudo systemctl start nitro-enclaves-allocator.service
sudo systemctl enable --now nitro-enclaves-vsock-proxy.service
``` 
or
```sh
vsock-proxy 8000 kms.east-northeast-1.amazonaws.com 443 &
```

### Anychain Ethereum
### Terraform AWS Provider
