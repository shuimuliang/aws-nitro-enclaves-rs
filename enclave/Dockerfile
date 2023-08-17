####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

ARG TARGETPLATFORM
# x86_64-unknown-linux-musl 
# aarch64-unknown-linux-musl

RUN rustup target add $TARGETPLATFORM
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --target $TARGETPLATFORM --release

####################################################################################################
## Final image
####################################################################################################
FROM arm64v8/alpine
ARG TARGETPLATFORM

WORKDIR /app

# Copy our build
COPY --from=builder /app/target/${TARGETPLATFORM}/release/enclave-server ./
# Copy kmstool_enclave_cli
COPY --from=builder /app/kmstool_enclave_cli /app/kmstool_enclave_cli
COPY --from=builder /app/libnsm.so /myip/libnsm.so
COPY --from=builder /app/libnsm.so /usr/lib64/libnsm.so

ENV REGION ap-east-1
ENV LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/lib64

RUN chmod +x /app/kmstool_enclave_cli

CMD ["./app/enclave-server", "-p", "7878"]
