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
        doxygen

if [ -d ~/neovim ]
then
        echo 'skip for now'
        # test maybe this is too much of removing for an update
        # sudo rm -rf /usr/local/bin/nvim
        # git -C ~/neovim pull
        # make -C ~/neovim distclean
        # make -C ~/neovim clean
        # make -C ~/neovim CMAKE_BUILD_TYPE=RelWithDebInfo
        # sudo make -C ~/neovim install
else
        git clone https://github.com/neovim/neovim ~/neovim
        make -C ~/neovim CMAKE_BUILD_TYPE=RelWithDebInfo
        sudo make -C ~/neovim install
fi

if [ ! -d ~/.local/share/nvim/site/pack/packer/start/packer.nvim ]
then
        git clone --depth 1 https://github.com/wbthomason/packer.nvim \
                ~/.local/share/nvim/site/pack/packer/start/packer.nvim
fi

if [ ! -d ~/.config ]; then
        # mkdir -p ~/.config/nvim
        # cp ../settings/init.lua ~/.config/nvim
        ln -s ~/victor-prokhorov/.config/ ~/.config 
fi

if [ -d ~/.fzf ]
then
        git -C ~/.fzf pull && yes | ~/.fzf/./install
else
        git clone --depth 1 https://github.com/junegunn/fzf.git ~/.fzf
        yes | ~/.fzf/install
fi


curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

if [ ! -d ~/.npm-global ]; then
        mkdir ~/.npm-global
        npm config set prefix '~/.npm-global'
        echo 'export PATH=~/.npm-global/bin:$PATH' >> ~/.profile
fi

npm i -g typescript typescript-language-server vscode-langservers-extracted eslint prettier

nvim --headless -c 'autocmd User PackerComplete quitall' -c 'PackerSync' #&> /dev/null