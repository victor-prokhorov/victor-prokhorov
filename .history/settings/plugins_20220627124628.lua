local use = require('packer').use

return require('packer').startup(function()
  use 'wbthomason/packer.nvim'
  use 'chriskempson/base16-vim'
  -- use 'neovim/nvim-lspconfig'
  use 'junegunn/fzf'
  use 'junegunn/fzf.vim'
  use 'neovim/nvim-lspconfig' -- Collection of configurations for built-in LSP client
  use 'hrsh7th/nvim-cmp' -- Autocompletion plugin
  use 'hrsh7th/cmp-nvim-lsp' -- LSP source for nvim-cmp
  use 'saadparwaiz1/cmp_luasnip' -- Snippets source for nvim-cmp
  -- use 'L3MON4D3/LuaSnip' -- Snippets plugin
  -- use 'neovim/nvim-lspconfig'
  -- use 'hrsh7th/cmp-nvim-lsp'
  use 'hrsh7th/cmp-buffer'
  use 'hrsh7th/cmp-path'
  use 'hrsh7th/cmp-cmdline'
  -- use 'hrsh7th/nvim-cmp'
  use 'L3MON4D3/LuaSnip' -- Snippets plugin
  use 'simrat39/rust-tools.nvim'
  use 'nvim-lua/plenary.nvim'
  use 'mfussenegger/nvim-dap'
  use 'hrsh7th/vscode-langservers-extracted'
-- autocomplete

use "windwp/nvim-autopairs"
use 'airblade/vim-rooter'
-- use { "windwp/nvim-autopairs", config = function() require("nvim-autopairs").setup {} end }
end)
