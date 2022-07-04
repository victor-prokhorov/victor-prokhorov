local cmd = vim.cmd
local use = require "packer".use

require "packer".startup(
    function()
        use "wbthomason/packer.nvim"
        use "neovim/nvim-lspconfig"
        use "junegunn/fzf"
        use "junegunn/fzf.vim"
        use "sumneko/lua-language-server"
        use "nvim-treesitter/nvim-treesitter"
        use "hrsh7th/cmp-nvim-lsp"
        use "hrsh7th/cmp-buffer"
        use "hrsh7th/cmp-path"
        use "hrsh7th/cmp-cmdline"
        use "hrsh7th/nvim-cmp"

        use "hrsh7th/cmp-vsnip"
        use "hrsh7th/vim-vsnip"

        -- use 'L3MON4D3/LuaSnip'
        -- use 'saadparwaiz1/cmp_luasnip'

        -- use 'f3fora/cmp-spell'
    end
)

local opts = {noremap = true, silent = true}
vim.keymap.set("n", "<space>e", vim.diagnostic.open_float, opts)
vim.keymap.set("n", "[d", vim.diagnostic.goto_prev, opts)
vim.keymap.set("n", "]d", vim.diagnostic.goto_next, opts)
vim.keymap.set("n", "<space>q", vim.diagnostic.setloclist, opts)

local on_attach = function(client, bufnr)
    vim.api.nvim_buf_set_option(bufnr, "omnifunc", "v:lua.vim.lsp.omnifunc")

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

require "lspconfig".sumneko_lua.setup {
    settings = {
        Lua = {
            runtime = {
                version = "LuaJIT"
            },
            diagnostics = {
                globals = {"vim"}
            },
            workspace = {
                library = vim.api.nvim_get_runtime_file("", true)
            },
            telemetry = {
                enable = false
            }
        }
    }
}

require "nvim-treesitter.configs".setup {
    -- A list of parser names, or "all"
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
    -- Install parsers synchronously (only applied to `ensure_installed`)
    sync_install = false,
    -- List of parsers to ignore installing (for "all")
    -- ignore_install = { "javascript" },
    highlight = {
        -- `false` will disable the whole extension
        enable = true,
        -- NOTE: these are the names of the parsers and not the filetype. (for example if you want to
        -- disable highlighting for the `tex` filetype, you need to include `latex` in this list as this is
        -- the name of the parser)
        -- list of language that will be disabled
        -- disable = { "c", "rust" },
        -- Setting this to true will run `:h syntax` and tree-sitter at the same time.
        -- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
        -- Using this option may slow down your editor, and you may see some duplicate highlights.
        -- Instead of true it can also be a list of languages
        additional_vim_regex_highlighting = false
    }
}

local cmp = require "cmp"

cmp.setup(
    {
        snippet = {
            -- REQUIRED - you must specify a snippet engine
            expand = function(args)
                vim.fn["vsnip#anonymous"](args.body) -- For `vsnip` users.
                -- require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
                -- require('snippy').expand_snippet(args.body) -- For `snippy` users.
                -- vim.fn["UltiSnips#Anon"](args.body) -- For `ultisnips` users.
            end
        },
        window = {},
        --       mapping = {
        --     -- Tab immediately completes. C-n/C-p to select.
        --     ['<Tab>'] = cmp.mapping.confirm({ select = true })
        --   },
        mapping = cmp.mapping.preset.insert(
            {
                ["<C-b>"] = cmp.mapping.scroll_docs(-4),
                ["<C-f>"] = cmp.mapping.scroll_docs(4),
                ["<C-Space>"] = cmp.mapping.complete(),
                ["<C-e>"] = cmp.mapping.abort(),
                ["<CR>"] = cmp.mapping.confirm({select = true}) -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
            }
        ),
        sources = cmp.config.sources(
            {
                {name = "nvim_lsp"},
                {name = "vsnip"} -- For vsnip users.
                --  { name = 'luasnip' }, -- For luasnip users.
                -- { name = 'ultisnips' }, -- For ultisnips users.
                -- { name = 'snippy' }, -- For snippy users.
            },
            {
                {name = "buffer"},
                -- { name = 'spell' },
                {name = "path"}
            }
        )
    }
)
-- vim.opt.spell = true
-- vim.opt.spelllang = { 'en_us' }
-- :spellgood <your word goes here>

-- Set configuration for specific filetype.
cmp.setup.filetype(
    "gitcommit",
    {
        sources = cmp.config.sources(
            {
                {name = "cmp_git"} -- You can specify the `cmp_git` source if you were installed it.
            },
            {
                {name = "buffer"}
            }
        )
    }
)

cmp.setup.cmdline(
    "/",
    {
        mapping = cmp.mapping.preset.cmdline(),
        sources = {
            {name = "buffer"}
        }
    }
)

require "lspconfig".tsserver.setup {}

cmd [[colorscheme slate]]
cmd [[set completeopt=menuone,noinsert,noselect]]
cmd [[set updatetime=300]]
cmd [[set number]]
cmd [[set autoindent]]
cmd [[set smartindent]]
cmd [[set nohlsearch]]
cmd [[set hidden]]
cmd [[set wildmenu]]
cmd [[set wildmode=list:longest]]
cmd [[set scrolloff=2]]
cmd [[set encoding=utf-8]]
cmd [[set incsearch]]
cmd [[set ignorecase]]
cmd [[set smartcase]]
cmd [[set gdefault]]
cmd [[set shiftwidth=2]]
cmd [[set softtabstop=2]]
cmd [[set tabstop=2]]
cmd [[set expandtab]]
cmd [[set mouse=a]]

local cmd = vim.cmd
local use = require "packer".use

require "packer".startup(
    function()
        use "wbthomason/packer.nvim"
        use "neovim/nvim-lspconfig"
        use "junegunn/fzf"
        use "junegunn/fzf.vim"
        use "sumneko/lua-language-server"
        use "nvim-treesitter/nvim-treesitter"
        use "hrsh7th/cmp-nvim-lsp"
        use "hrsh7th/cmp-buffer"
        use "hrsh7th/cmp-path"
        use "hrsh7th/cmp-cmdline"
        use "hrsh7th/nvim-cmp"

        use "hrsh7th/cmp-vsnip"
        use "hrsh7th/vim-vsnip"

        -- use 'L3MON4D3/LuaSnip'
        -- use 'saadparwaiz1/cmp_luasnip'

        -- use 'f3fora/cmp-spell'
    end
)

local opts = {noremap = true, silent = true}
vim.keymap.set("n", "<space>e", vim.diagnostic.open_float, opts)
vim.keymap.set("n", "[d", vim.diagnostic.goto_prev, opts)
vim.keymap.set("n", "]d", vim.diagnostic.goto_next, opts)
vim.keymap.set("n", "<space>q", vim.diagnostic.setloclist, opts)

local on_attach = function(client, bufnr)
    vim.api.nvim_buf_set_option(bufnr, "omnifunc", "v:lua.vim.lsp.omnifunc")

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

require "lspconfig".sumneko_lua.setup {
    settings = {
        Lua = {
            runtime = {
                version = "LuaJIT"
            },
            diagnostics = {
                globals = {"vim"}
            },
            workspace = {
                library = vim.api.nvim_get_runtime_file("", true)
            },
            telemetry = {
                enable = false
            }
        }
    }
}

require "nvim-treesitter.configs".setup {
    -- A list of parser names, or "all"
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
    -- Install parsers synchronously (only applied to `ensure_installed`)
    sync_install = false,
    -- List of parsers to ignore installing (for "all")
    -- ignore_install = { "javascript" },
    highlight = {
        -- `false` will disable the whole extension
        enable = true,
        -- NOTE: these are the names of the parsers and not the filetype. (for example if you want to
        -- disable highlighting for the `tex` filetype, you need to include `latex` in this list as this is
        -- the name of the parser)
        -- list of language that will be disabled
        -- disable = { "c", "rust" },
        -- Setting this to true will run `:h syntax` and tree-sitter at the same time.
        -- Set this to `true` if you depend on 'syntax' being enabled (like for indentation).
        -- Using this option may slow down your editor, and you may see some duplicate highlights.
        -- Instead of true it can also be a list of languages
        additional_vim_regex_highlighting = false
    }
}

local cmp = require "cmp"

cmp.setup(
    {
        snippet = {
            -- REQUIRED - you must specify a snippet engine
            expand = function(args)
                vim.fn["vsnip#anonymous"](args.body) -- For `vsnip` users.
                -- require('luasnip').lsp_expand(args.body) -- For `luasnip` users.
                -- require('snippy').expand_snippet(args.body) -- For `snippy` users.
                -- vim.fn["UltiSnips#Anon"](args.body) -- For `ultisnips` users.
            end
        },
        window = {},
        --       mapping = {
        --     -- Tab immediately completes. C-n/C-p to select.
        --     ['<Tab>'] = cmp.mapping.confirm({ select = true })
        --   },
        mapping = cmp.mapping.preset.insert(
            {
                ["<C-b>"] = cmp.mapping.scroll_docs(-4),
                ["<C-f>"] = cmp.mapping.scroll_docs(4),
                ["<C-Space>"] = cmp.mapping.complete(),
                ["<C-e>"] = cmp.mapping.abort(),
                ["<CR>"] = cmp.mapping.confirm({select = true}) -- Accept currently selected item. Set `select` to `false` to only confirm explicitly selected items.
            }
        ),
        sources = cmp.config.sources(
            {
                {name = "nvim_lsp"},
                {name = "vsnip"} -- For vsnip users.
                --  { name = 'luasnip' }, -- For luasnip users.
                -- { name = 'ultisnips' }, -- For ultisnips users.
                -- { name = 'snippy' }, -- For snippy users.
            },
            {
                {name = "buffer"},
                -- { name = 'spell' },
                {name = "path"}
            }
        )
    }
)
-- vim.opt.spell = true
-- vim.opt.spelllang = { 'en_us' }
-- :spellgood <your word goes here>

-- Set configuration for specific filetype.
cmp.setup.filetype(
    "gitcommit",
    {
        sources = cmp.config.sources(
            {
                {name = "cmp_git"} -- You can specify the `cmp_git` source if you were installed it.
            },
            {
                {name = "buffer"}
            }
        )
    }
)

cmp.setup.cmdline(
    "/",
    {
        mapping = cmp.mapping.preset.cmdline(),
        sources = {
            {name = "buffer"}
        }
    }
)

require "lspconfig".tsserver.setup {}

cmd [[colorscheme slate]]
cmd [[set completeopt=menuone,noinsert,noselect]]
cmd [[set updatetime=300]]
cmd [[set number]]
cmd [[set autoindent]]
cmd [[set smartindent]]
cmd [[set nohlsearch]]
cmd [[set hidden]]
cmd [[set wildmenu]]
cmd [[set wildmode=list:longest]]
cmd [[set scrolloff=2]]
cmd [[set encoding=utf-8]]
cmd [[set incsearch]]
cmd [[set ignorecase]]
cmd [[set smartcase]]
cmd [[set gdefault]]
cmd [[set shiftwidth=2]]
cmd [[set softtabstop=2]]
cmd [[set tabstop=2]]
cmd [[set expandtab]]
cmd [[set mouse=a]]

