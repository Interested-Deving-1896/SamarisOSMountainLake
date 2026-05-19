FROM --platform=linux/amd64 debian:trixie

ENV DEBIAN_FRONTEND=noninteractive
ENV PATH="/root/.cargo/bin:${PATH}"

RUN dpkg --add-architecture arm64 \
  && apt-get update \
  && apt-get install -y --no-install-recommends \
    ca-certificates \
    debootstrap \
    rsync \
    squashfs-tools \
    xorriso \
    isolinux \
    syslinux-common \
    grub-pc-bin \
    grub-efi-amd64-bin \
    grub-efi-arm64-bin:arm64 \
    mtools \
    unzip \
    curl \
    git \
    file \
    cmake \
    build-essential \
    pkg-config \
    gcc-aarch64-linux-gnu \
    g++-aarch64-linux-gnu \
    binutils-aarch64-linux-gnu \
    qemu-user-static \
    qemu-system-x86 \
    binfmt-support \
    plymouth \
    plymouth-themes \
    nodejs \
    npm \
  && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal --default-toolchain stable \
  && rustup target add aarch64-unknown-linux-gnu \
  && rustc --version \
  && cargo --version

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
ENV AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar
