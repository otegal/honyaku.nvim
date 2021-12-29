# Development
FROM rust:1.57.0-slim-buster AS dev

# ローカルのtargetディレクトリにビルドするとマウントしている時に遅くなるのでビルドディレクトリを変える
ENV CARGO_TARGET_DIR=/tmp/target \
    DEBIAN_FRONTEND=noninteractive \
    LC_CTYPE=ja_JP.utf8 \
    LANG=ja_JP.utf8

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     gnupg \
     apt-transport-https\
     libssl-dev \
     pkg-config \
     curl \
     build-essential \
     git \
     wget \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen \
  && echo "install rust tools" \
  && rustup component add rustfmt \
  && cargo install cargo-watch cargo-make \
  && chmod go-w /usr/local/cargo /usr/local/cargo/bin

RUN USER=root cargo new --lib app
WORKDIR /app
COPY ./Cargo.toml Cargo.toml
COPY ./Cargo.lock Cargo.lock
RUN cargo build --color never \
  && rm src/*.rs \
  && find $CARGO_TARGET_DIR/ -name "libhonyaku*" -delete \
  && find $CARGO_TARGET_DIR/ -name "honyaku*" -prune -exec rm -rf {} +

COPY . /app
RUN cargo build

CMD ["cargo", "run"]

# Build binaries for production
FROM dev AS build

RUN RUSTFLAGS="-C debuginfo=1" cargo build \
  --release --color never \
  --bin honyaku

# Production
FROM debian:buster-slim AS production

RUN apt-get update \
  && apt-get install -y -q \
     ca-certificates \
     locales \
     libpq-dev \
     gnupg \
     libssl-dev \
  && echo "ja_JP UTF-8" > /etc/locale.gen \
  && locale-gen

RUN mkdir -p /app/bin
WORKDIR /app

COPY --from=build /tmp/target/release/honyaku /app/bin

CMD ["/app/bin/honyaku"]
