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
    use 'L3MON4D3/LuaSnip'
    use 'nvim-lua/plenary.nvim'
    use 'mfussenegger/nvim-dap'
    -- use 'hrsh7th/cmp-nvim-lsp' -- LSP source for nvim-cmp
end)

cmd [[syntax on]]
cmd [[set termguicolors]]
-- cmd [[colorscheme base16-gruvbox-dark-hard]]
cmd [[colorscheme slate]]
cmd [[set background=dark]]
-- cmd [[call Base16hi("Comment", g:base16_gui09, "", g:base16_cterm09, "", "", "")]]
-- cmd [[call Base16hi("LspSignatureActiveParameter", g:base16_gui05, g:base16_gui03, g:base16_cterm05, g:base16_cterm03, "bold", "")]]
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

-- require('plugins')
require'lspconfig'.tsserver.setup {}
-- require('rust-tools').setup({})

-- require('nvim-autopairs').setup({})
-- require('nvim-autopairs').enable()
-- Setup nvim-cmp.
local cmp = require 'cmp'

cmp.setup({
    snippet = {
        -- REQUIRED - you must specify a snippet engine
        expand = function(args)
            vim.fn["vsnip#anonymous"](args.body) -- For `vsnip` users.
            -- require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
            -- require('snippy').expand_snippet(args.body) -- For `snippy` users.
            -- vim.fn["UltiSnips#Anon"](args.body) -- For `ultisnips` users.
        end
    },
    window = {
        -- completion = cmp.config.window.bordered(),
        -- documentation = cmp.config.window.bordered(),
    },
    mapping = cmp.mapping.preset.insert({
        ['<Tab>'] = cmp.mapping.confirm({
            select = true
        })
        -- ['<C-b>'] = cmp.mapping.scroll_docs(-4),
        -- ['<C-f>'] = cmp.mapping.scroll_docs(4),
        -- ['<C-Space>'] = cmp.mapping.complete(),
        -- ['<C-e>'] = cmp.mapping.abort(),
        -- ['<CR>'] = cmp.mapping.confirm({ select = true }), -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
    }),
    sources = cmp.config.sources({{
        name = 'nvim_lsp'
    }, {
        name = 'vsnip'
    } -- For vsnip users.
    -- { name = 'luasnip' }, -- For luasnip users.
    -- { name = 'ultisnips' }, -- For ultisnips users.
    -- { name = 'snippy' }, -- For snippy users.
    }, {{
        name = 'buffer'
    }})
})

-- Set configuration for specific filetype.
cmp.setup.filetype('gitcommit', {
    sources = cmp.config.sources({{
        name = 'cmp_git'
    } -- You can specify the `cmp_git` source if you were installed it.
    }, {{
        name = 'buffer'
    }})
})

-- Use buffer source for `/` (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline('/', {
    mapping = cmp.mapping.preset.cmdline(),
    sources = {{
        name = 'buffer'
    }}
})

-- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline(':', {
    mapping = cmp.mapping.preset.cmdline(),
    sources = cmp.config.sources({{
        name = 'path'
    }}, {{
        name = 'cmdline'
    }})
})

-- Setup lspconfig.
local capabilities = require('cmp_nvim_lsp').update_capabilities(vim.lsp.protocol.make_client_capabilities())
-- Replace <YOUR_LSP_SERVER> with each lsp server you've enabled.
require('lspconfig')['tsserver'].setup {
    capabilities = capabilities
}
-- add config for eslint
require'lspconfig'.eslint.setup {}
-- closing
local disable_filetype = {"TelescopePrompt"}
local disable_in_macro = false -- disable when recording or executing a macro
local disable_in_visualblock = false -- disable when insert after visual block mode
local ignored_next_char = [=[[%w%%%'%[%"%.]]=]
local enable_moveright = true
local enable_afterquote = true -- add bracket pairs after quote
local enable_check_bracket_line = true --- check bracket in same line
local enable_bracket_in_quote = true --
local break_undo = true -- switch for basic rule break undo sequence
local check_ts = false
local map_cr = true
local map_bs = true -- map the <BS> key
local map_c_h = false -- Map the <C-h> key to delete a pair
local map_c_w = false -- map <c-w> to delete a pair if possible

-- Add additional capabilities supported by nvim-cmp
local capabilities = vim.lsp.protocol.make_client_capabilities()
capabilities = require('cmp_nvim_lsp').update_capabilities(capabilities)

local lspconfig = require('lspconfig')

-- Enable some language servers with the additional completion capabilities offered by nvim-cmp
-- local servers = {'clangd', 'rust_analyzer', 'pyright', 'tsserver'}
local servers = {'tsserver'}
for _, lsp in ipairs(servers) do
    lspconfig[lsp].setup {
        -- on_attach = my_custom_on_attach,
        capabilities = capabilities
    }
end

-- luasnip setup
local luasnip = require 'luasnip'

-- nvim-cmp setup
local cmp = require 'cmp'
cmp.setup {
    snippet = {
        expand = function(args)
            luasnip.lsp_expand(args.body)
        end
    },
    mapping = cmp.mapping.preset.insert({
        ['<C-d>'] = cmp.mapping.scroll_docs(-4),
        ['<C-f>'] = cmp.mapping.scroll_docs(4),
        ['<C-Space>'] = cmp.mapping.complete(),
        ['<CR>'] = cmp.mapping.confirm {
            behavior = cmp.ConfirmBehavior.Replace,
            select = true
        },
        ['<Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
                cmp.select_next_item()
            elseif luasnip.expand_or_jumpable() then
                luasnip.expand_or_jump()
            else
                fallback()
            end
        end, {'i', 's'}),
        ['<S-Tab>'] = cmp.mapping(function(fallback)
            if cmp.visible() then
                cmp.select_prev_item()
            elseif luasnip.jumpable(-1) then
                luasnip.jump(-1)
            else
                fallback()
            end
        end, {'i', 's'})
    }),
    sources = {{
        name = 'nvim_lsp'
    }, {
        name = 'luasnip'
    }}
}

