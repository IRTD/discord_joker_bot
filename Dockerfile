FROM rust:1.70.0
WORKDIR /bot
COPY . .
RUN cargo install --path .
CMD ["joker-bot"]