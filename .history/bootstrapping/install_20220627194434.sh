#!/usr/bin/env bash

sudo apt update && sudo apt install -y \
        build-essential \
        cmake \
        pkg-config \
        libtool \
        libtool-bin \
        unzip \
        gettext

git clone https://github.com/neovim/neovim
make -C neovim CMAKE_BUILD_TYPE=RelWithDebInfo
sudo make -C neovim install

git clone --depth 1 https://github.com/wbthomason/packer.nvim \
        ~/.local/share/nvim/site/pack/packer/start/packer.nvim
mkdir -p ~/.config/nvim/lua
cp ../settings/init.vim ~/.config/nvim
cp ../settings/plugins.lua ~/.config/nvim/lua

git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
yes | ~/.fzf/install
source ~/.bashrc

nvim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync' # &> /dev/null
echo 

curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# WSL 2 double check on normal
mkdir ~/.npm-global
npm config set prefix '~/.npm-global'
export PATH=~/.npm-global/bin:$PATH
source ~/.profile
# npm i -g typescript-language-server