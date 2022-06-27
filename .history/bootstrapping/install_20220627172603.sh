#!/usr/bin/env bash

apt update && apt install -y \
        build-essential \
        cmake \
        pkg-config \
        libtool \
        libtool-bin \
        unzip \
        gettext

git clone https://github.com/neovim/neovim
make -C neovim CMAKE_BUILD_TYPE=RelWithDebInfo
make -C neovim install

git clone --depth 1 https://github.com/wbthomason/packer.nvim \
        .local/share/nvim/site/pack/packer/start/packer.nvim

mkdir -p .config/nvim/lua

nvim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync'
cp ../settings/init.vim $HOME/