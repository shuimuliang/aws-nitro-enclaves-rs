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
### Nitro Enclaves vsock client
### Nitro Enclaves vsock server
### Socket Protocol helpers
### AWS IAM
### AWS DynamoDB
### AWS KMS
### Anychain Ethereum
### Terraform AWS Provider
