FROM rust AS builder
LABEL maintainer="extrawurst"
WORKDIR bot
ADD src ./src
ADD Cargo.toml ./Cargo.toml
ADD Cargo.lock ./Cargo.lock
RUN cargo build --release
RUN cp ./target/release/gr-discord-bot ./target/bot

FROM ubuntu
LABEL maintainer="extrawurst"
RUN apt-get update && apt-get install -y openssl
WORKDIR gr-bot
COPY --from=builder /bot/target/bot ./
CMD ["./bot"]