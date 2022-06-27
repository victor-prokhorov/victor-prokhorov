#!/usr/bin/env bash

apt update && apt install -y \
  cmake \
  pkg-config \
  libtool \
  libtool-bin \
  unzip

  mkdir -p 