local cmd = vim.cmd
local use = require('packer').use

require('packer').startup(function()
    use 'wbthomason/packer.nvim'
    use 'airblade/vim-rooter'
    use 'junegunn/fzf'
    use 'junegunn/fzf.vim'
    use 'editorconfig/editorconfig-vim'
    use 'itchyny/lightline.vim'
    use "windwp/nvim-autopairs"
    use 'neovim/nvim-lspconfig'
    use 'nvim-lua/lsp_extensions.nvim'
    use {
        'hrsh7th/cmp-nvim-lsp',
        branch = 'main'
    }
    use {
        'hrsh7th/cmp-buffer',
        branch = 'main'
    }
    use {
        'hrsh7th/cmp-path',
        branch = 'main'
    }
    use {
        'hrsh7th/nvim-cmp',
        branch = 'main'
    }
    use 'ray-x/lsp_signature.nvim'
    use {
        'hrsh7th/cmp-vsnip',
        branch = 'main'
    }
    use 'hrsh7th/vim-vsnip'
    use {
        'cespare/vim-toml',
        branch = 'main'
    }
    use 'stephpy/vim-yaml'
    use 'rust-lang/rust.vim'
    use 'godlygeek/tabular'
    use 'plasticboy/vim-markdown'
    use 'chriskempson/base16-vim'
    use 'hrsh7th/vscode-langservers-extracted'
    ---
end)

cmd [[syntax on]]
cmd [[set termguicolors]]
cmd [[colorscheme base16-gruvbox-dark-hard]]
cmd [[set background=dark]]
cmd [[call Base16hi("Comment", g:base16_gui09, "", g:base16_cterm09, "", "", "")]]
cmd [[call Base16hi("LspSignatureActiveParameter", g:base16_gui05, g:base16_gui03, g:base16_cterm05, g:base16_cterm03, "bold", "")]]
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

require'nvim-autopairs'.setup {}
