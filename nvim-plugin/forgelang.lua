-- ForgeLang Tree-sitter parser for Neovim
-- Install with: :TSInstallFromGrammar forgelang

local M = {}

function M.setup()
  -- Add ForgeLang file extension
  vim.filetype.add({
    extension = {
      fl = 'forgelang',
    },
  })

  -- Set up tree-sitter from grammar
  require('nvim-treesitter.parsers').get_parser_configs().forgelang = {
    install_info = {
      type = 'local',
      path = vim.fn.expand('~/.config/nvim/pack/plugins/start/tree-sitter-forgelang'),
      files = {'src/parser.c'},
      branch = 'main',
    },
    filetype = 'forgelang',
  }
end

return M
