FROM rust:1.65.0-slim-buster as build

RUN apt update 
RUN apt install -qq -y --no-install-recommends 
RUN apt install -y build-essential libssl-dev pkg-config libpq-dev

RUN USER=root cargo new --bin holy
WORKDIR /app

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release
RUN ls -la
RUN rm -rf ./target/release/deps/
RUN rm src/*.rs

FROM debian:buster-slim
RUN apt update 
RUN apt install -qq -y --no-install-recommends 
RUN apt install -qq -y build-essential libssl-dev pkg-config libpq-dev

WORKDIR /usr/app

COPY flag.txt /tmp
RUN mv /tmp/flag.txt /
RUN chmod +r /flag.txt 
RUN mv /flag.txt /$(cat /dev/urandom | tr -cd 'a-f0-9' | head -c 32)
COPY --from=build /app/target/release/backend .






# Create the user
RUN groupadd --gid 1000 server && useradd --uid 1000 --gid 1000 -m server

RUN chown server /usr/app/
USER server
RUN mkdir migrations
RUN mkdir scripts
RUN mkdir -p static/covers static/songs
COPY /static/songs static/songs
COPY /static/covers static/covers
COPY /migrations ./migrations/
COPY /scripts ./scripts


RUN mkdir sessions



CMD ["./backend"]

EXPOSE 3000
