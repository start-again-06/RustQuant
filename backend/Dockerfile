FROM rust:1.75

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 3000

CMD ["./target/release/rustquantai-backend"]
