# aws-nitro-enclaves-rs

## Architecture Diagram
[Encrypted Wallet Application Based on Nitro Enclaves + AWS EKS](https://aws.amazon.com/cn/blogs/china/crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks/)

build a secure and trusted execution environment based on nitro enclave3
![build-a-secure-and-trusted-execution-environment-based-on-nitro-enclave3.png](docs%2Fimages%2Fbuild-a-secure-and-trusted-execution-environment-based-on-nitro-enclave3.png)

## Generate Account
![crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks2.png](docs%2Fimages%2Fcrypto-wallet-application-based-on-nitro-enclaves-and-aws-eks2.png)

### Account Generation Workflow

1. The parent instance client receives the generateAccount API call.
2. The getlAMToken() function is called to obtain IAM Role credentials.
3. The credentials are sent via vsock and a request for the generateAccount API call is made.
4. An encrypted wallet account is generated in Nitro Enclave.
5. Credentials and attestation are used to request the KMS generateDataKey API call.
6. Account information is encrypted using the datakey.
7. The encrypted account information is sent to the parent instance via vsock.
8. The API is called to save the encrypted account information to DynamoDB.

## Sign Signature by Private Key
![crypto-wallet-application-based-on-nitro-enclaves-and-aws-eks3.png](docs%2Fimages%2Fcrypto-wallet-application-based-on-nitro-enclaves-and-aws-eks3.png)

### Transaction Signature Workflow

1. The parent instance client receives a sign API call.
2. The getlAMToken() function is called to obtain IAM Role credentials.
3. The credentials are sent via vsock and the API call is signed.
4. The encrypted data key is decrypted using the KMS API.
5. The encrypted wallet private key is decrypted using the data key.
6. The decrypted wallet private key is used to sign the message.
7. The signature is sent back to the parent instance via vsock.
 
## Core Components
### Nitro Enclaves vsock client
### Nitro Enclaves vsock server
### Socket Protocol helpers
### AWS IAM
### AWS DynamoDB
### AWS KMS
### Anychain Ethereum
