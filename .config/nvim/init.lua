local use = require "packer".use
require "packer".startup(
    function()
        use "wbthomason/packer.nvim"
        use "nvim-treesitter/nvim-treesitter"
        use "neovim/nvim-lspconfig"
        use "hrsh7th/cmp-nvim-lsp"
        use "hrsh7th/cmp-buffer"
        use "hrsh7th/cmp-path"
        use "hrsh7th/cmp-cmdline"
        use "hrsh7th/nvim-cmp"
        use "hrsh7th/cmp-vsnip"
        use "hrsh7th/vim-vsnip"
        use "junegunn/fzf"
        use "junegunn/fzf.vim"
        use "windwp/nvim-autopairs"
        use "base16-project/base16-vim"
        use "jnurmine/Zenburn" -- nice but need adjust lsp diagnostics
        use 'rust-lang/rust.vim'
    end
)

local cmd = vim.cmd
vim.g.completeopt = "menu,menuone,noinsert,noselect"

cmd "set noswapfile"
cmd "set signcolumn=yes" -- always draw the column for diagnostics signs 
cmd "set scrolloff=10"
-- cmd "set laststatus=2"
cmd "set clipboard=unnamed" -- share buffer between neovim and tmux
cmd "set timeoutlen=150"
cmd "set nocompatible"
cmd "filetype off"
cmd "set termguicolors"
cmd "colorscheme base16-gruvbox-dark-hard"
-- cmd "colorscheme zenburn"
cmd "set updatetime=150" -- diagnostic message
cmd "set number"
cmd "set autoindent"
cmd "set smartindent"
cmd "set hlsearch"
cmd "set hidden"
cmd "set wildmenu"
cmd "set wildmode=list:longest"
cmd "set scrolloff=2"
cmd "set encoding=utf-8"
cmd "set incsearch"
cmd "set ignorecase"
cmd "set smartcase"
cmd "set gdefault"
cmd "set shiftwidth=2"
cmd "set softtabstop=2"
cmd "set tabstop=2"
cmd "set expandtab"
cmd "set mouse=a"
cmd "syntax enable" -- required by rust.vim
cmd "filetype plugin indent on" -- required by rust.vim

require "nvim-autopairs".setup {
    disable_in_macro = false,
    disable_in_visualblock = false,
    ignored_next_char = [=[[%w%%%'%[%"%.]]=],
    enable_moveright = true,
    enable_afterquote = true,
    enable_check_bracket_line = true,
    enable_bracket_in_quote = true,
    break_undo = true,
    check_ts = false,
    map_cr = true,
    map_bs = true,
    map_c_h = false,
    map_c_w = false
}

require "nvim-treesitter.configs".setup {
    ensure_installed = {
        "lua",
        "bash",
        "yaml",
        "rust",
        "toml",
        "dockerfile",
        "javascript",
        "typescript",
        "graphql",
        "make",
        "markdown",
        "ninja",
        "html",
        "css",
        "jsdoc",
        "json",
        "tsx"
    },
    sync_install = false,
    highlight = {enable = true, additional_vim_regex_highlighting = false},
    indent = {
        enable = true
    }
}
local cmp_autopairs = require "nvim-autopairs.completion.cmp"

local cmp = require "cmp"

cmp.event:on("confirm_done", cmp_autopairs.on_confirm_done())

cmp.setup(
    {
        snippet = {
            expand = function(args)
                vim.fn["vsnip#anonymous"](args.body)
            end
        },
        mapping = {
            -- ["<Tab>"] = cmp.mapping.confirm({select = true}),
            ['<TAB>'] = cmp.mapping(
        { 
          i = cmp.mapping.confirm({ behavior = cmp.ConfirmBehavior.Insert, select = true }),
          c = cmp.mapping(cmp.mapping.select_next_item())
        }),
       
            ["<C-p>"] = cmp.mapping.select_prev_item(),
            ["<C-n>"] = cmp.mapping.select_next_item()
        },
        -- formatting = {
        --   fields = { 'menu', 'abbr', 'kind'}
        -- },
        sources = cmp.config.sources(
            {
                {name = "nvim_lsp"},
                {name = "vsnip"},
                {name = "path"},
                {name = "buffer"}
            }
            -- {
            --
            -- }
        ),
        experimental = {
            native_menu = false,
            ghost_text = true
        }
    }
)

-- Set configuration for specific filetype.
cmp.setup.filetype(
    "gitcommit",
    {
        sources = cmp.config.sources(
            {
                {name = "cmp_git"} -- You can specify the `cmp_git` source if you were installed it.
            },
            {{name = "buffer"}}
        )
    }
)

-- Use buffer source for `/` (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline(
    "/",
    {
        mapping = cmp.mapping.preset.cmdline(),
        sources = {{name = "buffer"}}
    }
)

-- Use cmdline & path source for ':' (if you enabled `native_menu`, this won't work anymore).
cmp.setup.cmdline(
    ":",
    {
        mapping = cmp.mapping.preset.cmdline(),
        sources = cmp.config.sources({{name = "path"}}, {{name = "cmdline"}})
    }
)

-- Mappings.
-- See `:help vim.diagnostic.*` for documentation on any of the below functions
local opts = {noremap = true, silent = true}
vim.keymap.set("n", "<space>e", vim.diagnostic.open_float, opts)
vim.keymap.set("n", "[d", vim.diagnostic.goto_prev, opts)
vim.keymap.set("n", "]d", vim.diagnostic.goto_next, opts)
vim.keymap.set("n", "<space>q", vim.diagnostic.setloclist, opts)

-- Use an on_attach function to only map the following keys
-- after the language server attaches to the current buffer
local on_attach = function(client, bufnr)
    -- Enable completion triggered by <c-x><c-o>
    vim.api.nvim_buf_set_option(bufnr, "omnifunc", "v:lua.vim.lsp.omnifunc")

    -- Mappings.
    -- See `:help vim.lsp.*` for documentation on any of the below functions
    local bufopts = {noremap = true, silent = true, buffer = bufnr}
    vim.keymap.set("n", "gD", vim.lsp.buf.declaration, bufopts)
    vim.keymap.set("n", "gd", vim.lsp.buf.definition, bufopts)
    vim.keymap.set("n", "K", vim.lsp.buf.hover, bufopts)
    vim.keymap.set("n", "gi", vim.lsp.buf.implementation, bufopts)
    vim.keymap.set("n", "<C-k>", vim.lsp.buf.signature_help, bufopts)
    vim.keymap.set("n", "<space>wa", vim.lsp.buf.add_workspace_folder, bufopts)
    vim.keymap.set("n", "<space>wr", vim.lsp.buf.remove_workspace_folder, bufopts)
    vim.keymap.set(
        "n",
        "<space>wl",
        function()
            print(vim.inspect(vim.lsp.buf.list_workspace_folders()))
        end,
        bufopts
    )
    vim.keymap.set("n", "<space>D", vim.lsp.buf.type_definition, bufopts)
    vim.keymap.set("n", "<space>rn", vim.lsp.buf.rename, bufopts)
    vim.keymap.set("n", "<space>ca", vim.lsp.buf.code_action, bufopts)
    vim.keymap.set("n", "gr", vim.lsp.buf.references, bufopts)
    vim.keymap.set("n", "<space>f", vim.lsp.buf.formatting, bufopts)
end

local lsp_flags = {debounce_text_changes = 150}

local capabilities = require "cmp_nvim_lsp".update_capabilities(vim.lsp.protocol.make_client_capabilities())
capabilities.textDocument.completion.completionItem.snippetSupport = true

local lspconfig = require "lspconfig"

require "lspconfig".html.setup {
    on_attach = on_attach,
    flags = lsp_flags,
    capabilities = capabilities
}

require "lspconfig".cssls.setup {
    on_attach = on_attach,
    flags = lsp_flags,
    capabilities = capabilities
}

require "lspconfig".tsserver.setup {
    on_attach = on_attach,
    flags = lsp_flags,
    capabilities = capabilities
}

require "lspconfig".eslint.setup {
    on_attach = on_attach,
    flags = lsp_flags,
    capabilities = capabilities
}

require "lspconfig".rust_analyzer.setup {
  on_attach = on_attach,
  flags = lsp_flags,
  settings = {
    ["rust-analyzer"] = {
      cargo = {
        allFeatures = true,
      },
      completion = {
	postfix = {
	  enable = false,
	},
      },
    },
  },
  capabilities = capabilities,
}

cmd "autocmd BufWritePre *.tsx,*.ts,*.jsx,*.js EslintFixAll"
cmd "autocmd BufWritePre *.rs lua vim.lsp.buf.formatting_sync(nil, 1000)"
