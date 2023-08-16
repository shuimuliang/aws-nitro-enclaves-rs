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

How to build:
```sh
cd enclave
make server
```

### parent client

### AWS IAM
#### IAM Role
you need create a IAM Role which will be attached to EC2/EKS, it need have the access for kms and dynamodb. you need update this policy after your enclave image created with condition check of PCR
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
            "Resource": "your kms arn"
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

### AWS DynamoDB
Table name
- AccountTable

Colume
- KeyId: kms key id which used for encryption on the wallet private key
- Name: account name for this account, used for identify wallet
- EncryptedPrivateKey: encrypted wallet private key
- Address: the address key of the wallet
- EncryptedDataKey: the data key used to encrypt the private key
 
### AWS KMS

#### kms key
you need create a **Symmetric** kms key, which used for **Encrypt and decrypt**, you need copy this
```sh

```

### Anychain Ethereum
### Terraform AWS Provider
