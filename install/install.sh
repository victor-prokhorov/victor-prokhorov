#!/usr/bin/env bash

sudo apt update && sudo apt install -y \
        ninja-build \
        gettext \
        libtool \
        libtool-bin \
        autoconf \
        automake \
        cmake \
        g++ \
        pkg-config \
        unzip \
        curl \
        doxygen \
        ripgrep

if [ -d ~/neovim ]
then
        sudo rm -rf /usr/local/bin/nvim
        git -C ~/neovim pull
        make -C ~/neovim distclean
        make -C ~/neovim clean
        make -C ~/neovim CMAKE_BUILD_TYPE=RelWithDebInfo
        sudo make -C ~/neovim install
else
        git clone https://github.com/neovim/neovim ~/neovim
        make -C ~/neovim CMAKE_BUILD_TYPE=RelWithDebInfo
        sudo make -C ~/neovim install
fi

if [ ! -d ~/.local/share/nvim/site/pack/packer/start/packer.nvim ]; then
        git clone --depth 1 https://github.com/wbthomason/packer.nvim \
                ~/.local/share/nvim/site/pack/packer/start/packer.nvim
fi

if [ ! -d ~/.config ]; then
        ln -s ~/victor-prokhorov/.config/ ~/.config 
fi

if [ ! -d ~/.tmux.conf ]; then
        ln -s ~/victor-prokhorov/install/.tmux.conf ~/.tmux.conf 
fi



if [ -d ~/.fzf ]
then
        git -C ~/.fzf pull && yes | ~/.fzf/./install
else
        git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
        yes | ~/.fzf/install
fi

sudo apt remove nodejs 
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
# TODO: this stack
sudo apt install -y nodejs

if [ ! -d ~/.npm-global ]; then
        mkdir ~/.npm-global
        npm config set prefix '~/.npm-global'
        echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.profile
fi

# TODO: this stack as well
npm i npm@latest -g
npm i -g typescript typescript-language-server vscode-langservers-extracted eslint prettier
npm update -g


# timeout this too
nvim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync' &> /dev/null
# https://rust-lang.github.io/rustup/installation/index.html

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y
# probably need to source
source ~/.profile
rustup toolchain install nightly --allow-downgrade --profile minimal --component clippy rustfmt
# rustup component add rustfmt
# rustup component add clippy

# https://rust-analyzer.github.io/manual.html#installation
git clone https://github.com/rust-analyzer/rust-analyzer.git ~/rust-analyzer && cd ~/rust-analyzer
cargo xtask install --server

# !
# this laso include things included early edit
rm -f ~/.bashrc 
ln -s ~/victor-prokhorov/install/.bashrc ~/.bashrc 
