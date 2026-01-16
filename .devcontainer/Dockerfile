FROM ubuntu:22.04
    
# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    make \
    verilator \
    git \
    locales \
    && locale-gen en_US.UTF-8 \
    && update-locale LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8 \
    && rm -rf /var/lib/apt/lists/*

ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8
