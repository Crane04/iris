FROM rust:latest


RUN apt-get update && apt-get install -y \
    clang \
    llvm-dev \
    libclang-dev \
    pkg-config \
    libopencv-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src


RUN cargo build --release


COPY setup.sh ./setup.sh
RUN chmod +x setup.sh && ./setup.sh

EXPOSE 8080

CMD ["./target/release/iris"]