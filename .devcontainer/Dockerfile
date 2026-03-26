FROM mcr.microsoft.com/devcontainers/rust:latest

RUN apt update && apt upgrade -y && apt install -y qemu-system dosfstools mtools && rustup target add x86_64-unknown-uefi
