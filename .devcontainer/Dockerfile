FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y \
   # && apt-get install -y libapr1 libapr1-dev libaprutil1 libaprutil1-dev \
   # && sudo apt-get install libcurl4-openssl-dev libjson-c-dev cmake binutils make \
   # && git clone https://github.com/smartnode/telebot.git \
   # && cd telebot \
   # && mkdir -p Build && cd Build \
   # && cmake ../ \
   # && make
