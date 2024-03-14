FROM debian:bullseye

MAINTAINER Tanguy ROUDAUT <tanguy@roudaut.xyz>
LABEL version="1.0.0"
LABEL description="This is a docker image for a package building of the Smart-boat-stub GUI for debian bullseye"

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    gcc \
    libssl-dev \
    pkg-config 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN mkdir -p /root/smart-boat

WORKDIR /root/smart-boat

ENTRYPOINT ["/bin/sh", "/root/smart-boat/package-builder/debian/bd_smart-boat-gui_debian.sh"]