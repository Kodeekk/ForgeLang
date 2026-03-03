-- ╔══════════════════════════════════════════════════════════════════╗
-- ║              ForgeLang Neovim Configuration                       ║
-- ║                  Copy to ~/.config/nvim/                          ║
-- ╚══════════════════════════════════════════════════════════════════╝

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 1. INSTALL TREE-SITTER GRAMMAR
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- Run these commands in Neovim:
-- :TSInstallFromGrammar forgelang
-- Or manually:
-- cd ~/RustroverProjects/ForgeLang/tree-sitter-forgelang
-- npm install
-- tree-sitter generate
-- tree-sitter build --wasm
-- cp tree-sitter-forgelang.wasm ~/.local/share/nvim/lazy/nvim-treesitter/parser/

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 2. FILETYPE DETECTION
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
vim.filetype.add({
  extension = {
    fl = 'forgelang',
  },
  pattern = {
    ['.*%.fl$'] = 'forgelang',
  },
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 3. TREE-SITTER CONFIGURATION
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
require('nvim-treesitter.configs').setup({
  ensure_installed = {
    'forgelang',  -- Add after installing the grammar
    'rust',
    'lua',
    'markdown',
  },

  highlight = {
    enable = true,
    additional_vim_regex_highlighting = false,
    custom_captures = {
      ['keyword.storage'] = 'Keyword',
      ['type.builtin'] = 'Type',
      ['function.builtin'] = 'Function',
    },
  },

  indent = {
    enable = true,
  },

  incremental_selection = {
    enable = true,
    keymaps = {
      init_selection = '<C-space>',
      node_incremental = '<C-space>',
      scope_incremental = '<C-s>',
      node_decremental = '<bs>',
    },
  },
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 4. LSP CONFIGURATION (if you have an LSP server)
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
local lspconfig = require('lspconfig')

-- If you create a ForgeLang LSP server:
-- lspconfig.forgelang.setup({
--   cmd = { 'forgelang-lsp' },
--   filetypes = { 'forgelang' },
--   root_dir = lspconfig.util.root_pattern('forgelang.toml', '.git'),
--   settings = {
--     forgelang = {
--       diagnostics = {
--         enable = true,
--       },
--       completion = {
--         enable = true,
--       },
--     },
--   },
-- })

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 5. AUTOCOMMANDS
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
vim.api.nvim_create_autocmd('FileType', {
  pattern = 'forgelang',
  callback = function()
    -- Set local options for ForgeLang files
    vim.opt_local.tabstop = 4
    vim.opt_local.shiftwidth = 4
    vim.opt_local.softtabstop = 4
    vim.opt_local.expandtab = true
    vim.opt_local.textwidth = 100
    
    -- Keybindings for ForgeLang
    vim.keymap.set('n', '<leader>r', ':!ForgeLang %<CR>', { buffer = true, desc = 'Run ForgeLang file' })
    vim.keymap.set('n', '<leader>b', ':!ForgeLang build<CR>', { buffer = true, desc = 'Build ForgeLang project' })
  end,
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 6. COMPLETION (nvim-cmp)
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
local cmp = require('cmp')

cmp.setup({
  sources = cmp.config.sources({
    { name = 'nvim_lsp' },
    { name = 'treesitter' },
    { name = 'buffer' },
    { name = 'path' },
  }, {
    { name = 'forgelang' },  -- If you create a cmp source
  }),
  
  mapping = cmp.mapping.preset.insert({
    ['<C-b>'] = cmp.mapping.scroll_docs(-4),
    ['<C-f>'] = cmp.mapping.scroll_docs(4),
    ['<C-Space>'] = cmp.mapping.complete(),
    ['<C-e>'] = cmp.mapping.abort(),
    ['<CR>'] = cmp.mapping.confirm({ select = true }),
  }),
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 7. DIAGNOSTICS
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
vim.diagnostic.config({
  virtual_text = true,
  signs = true,
  underline = true,
  update_in_insert = false,
  severity_sort = true,
  float = {
    focusable = false,
    style = 'minimal',
    border = 'rounded',
    source = 'always',
    header = '',
    prefix = '',
  },
})

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 8. COLORS (for your colorscheme)
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
vim.api.nvim_set_hl(0, '@comment.forgelang', { fg = '#6a9955', italic = true })
vim.api.nvim_set_hl(0, '@string.forgelang', { fg = '#ce9178' })
vim.api.nvim_set_hl(0, '@number.forgelang', { fg = '#b5cea8' })
vim.api.nvim_set_hl(0, '@keyword.forgelang', { fg = '#569cd6', bold = true })
vim.api.nvim_set_hl(0, '@type.forgelang', { fg = '#4ec9b0' })
vim.api.nvim_set_hl(0, '@function.forgelang', { fg = '#dcdcaa' })
vim.api.nvim_set_hl(0, '@variable.forgelang', { fg = '#9cdcfe' })
vim.api.nvim_set_hl(0, '@operator.forgelang', { fg = '#d4d4d4' })

-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- 9. INSTALLATION SCRIPT
-- ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
-- Run this function once to set everything up:
local function setup_forgelang()
  print('Setting up ForgeLang support...')
  
  -- Install tree-sitter grammar
  local grammar_path = vim.fn.expand('~/RustroverProjects/ForgeLang/tree-sitter-forgelang')
  if vim.fn.isdirectory(grammar_path) == 1 then
    print('Found ForgeLang grammar at: ' .. grammar_path)
    
    -- Generate parser
    vim.fn.system('cd ' .. grammar_path .. ' && tree-sitter generate')
    print('Generated parser')
    
    -- Copy to nvim-treesitter
    local parser_dest = vim.fn.stdpath('data') .. '/lazy/nvim-treesitter/parser/forgelang.so'
    vim.fn.mkdir(vim.fn.fnamemodify(parser_dest, ':h'), 'p')
    vim.fn.system('cp ' .. grammar_path .. '/build/Release/tree-sitter_forgelang.so ' .. parser_dest)
    print('Installed parser to: ' .. parser_dest)
  else
    print('ERROR: Grammar not found at: ' .. grammar_path)
  end
  
  print('ForgeLang setup complete! Restart Neovim.')
end

-- Run with: :lua require('forgelang-setup').setup_forgelang()
return { setup_forgelang = setup_forgelang }
