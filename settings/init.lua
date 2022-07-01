local cmd = vim.cmd
local use = require'packer'.use

require'packer'.startup(function()
    use 'wbthomason/packer.nvim'
    use 'junegunn/fzf'
    use 'junegunn/fzf.vim'
end)

cmd [[syntax on]]
cmd [[set termguicolors]]
cmd [[colorscheme slate]]
cmd [[set background=dark]]
cmd [[set noshowmode]]
cmd [[set encoding=utf-8]]
cmd [[set hidden]]
cmd [[set nowrap]]
cmd [[set nojoinspaces]]
cmd [[set signcolumn=yes]]
cmd [[set wildmenu]]
cmd [[set wildmode=list:longest]]
cmd [[set shiftwidth=8]]
cmd [[set softtabstop=8]]
cmd [[set tabstop=8]]
cmd [[set noexpandtab]]
cmd [[set incsearch]]
cmd [[set ignorecase]]
cmd [[set smartcase]]
cmd [[set gdefault]]
cmd [[set number]]
cmd [[set showcmd]]
cmd [[set mouse=a]]
cmd [[set completeopt=menuone,noinsert,noselect]]
cmd [[set cmdheight=2]]
cmd [[set autoindent]]
cmd [[set smartindent]]

