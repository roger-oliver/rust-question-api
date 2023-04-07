FROM rust:latest AS builder

RUN apt -y update
RUN apt upgrade -y
RUN apt install libssl-dev
RUN apt install pkg-config
RUN rustup target add x86_64-unknown-linux-musl
RUN apt install -y musl-tools musl-dev
RUN apt install -y build-essential
RUN apt install -y gcc-x86-64-linux-gnu
# Install SSL certificates and dependencies for reqwest
RUN apt-get update && apt-get install -y ca-certificates libssl-dev

WORKDIR /app

COPY ./ .

ENV OPENSSL_DIR="/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_LIB_DIR="/usr/lib/x86_64-linux-gnu"
ENV RUSTFLAGS='-C linker=x86_64-linux-gnu-gcc'
ENV CC='gcc'
ENV CC_x86_64_unknown_linux_musl=x86_64-linux-gnu-gcc
ENV CC_x86_64-unknown-linux-musl=x86_64-linux-gnu-gcc

RUN cargo build --target x86_64-unknown-linux-musl --release

FROM scratch

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rust-question-api ./
COPY --from=builder /app/.env.docker.compose ./

# Copy the root CA certificates bundle and set the SSL_CERT_FILE environment variable
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt

CMD ["/app/rust-question-api"]
