# Provided without testing !

FROM node:latest

RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install -yq libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
RUN curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh -s -- -y
RUN source "$HOME/.cargo/env"
RUN cargo install tauri-cli

WORKDIR /home/node
