local M = {}

function M.setup()
  vim.g.mapleader = " "
  -- LSP
  local lspconfig = require("lspconfig")
  lspconfig.rust_analyzer.setup {
    capabilities = require("cmp_nvim_lsp").default_capabilities(),
    on_attach = function(_, bufnr)
	    local opts = { buffer = bufnr, noremap = true, silent = true }

	    -- LSP keymaps
	    vim.keymap.set("n", "gd", vim.lsp.buf.definition, opts)
	    vim.keymap.set("n", "gr", vim.lsp.buf.references, opts)
	    vim.keymap.set("n", "K", vim.lsp.buf.hover, opts)
	    vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, opts)
	    vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, opts)
	    vim.keymap.set("n", "<leader>f", function()
	      vim.lsp.buf.format { async = true }
	    end, opts)
    end,
  }

  -- Treesitter
  require("nvim-treesitter.configs").setup {
    highlight = { enable = true },
  }

  -- Completion
  local cmp = require("cmp")
  cmp.setup {
    mapping = cmp.mapping.preset.insert({
      ["<CR>"] = cmp.mapping.confirm({ select = true }),
      ["<C-Space>"] = cmp.mapping.complete(),
    }),
    sources = {
      { name = "nvim_lsp" },
    },
  }
  -- Telescope

  local builtin = require("telescope.builtin")
  vim.keymap.set("n", "<leader>ff", builtin.find_files, {})
  vim.keymap.set("n", "<leader>fg", builtin.live_grep, {})
  vim.keymap.set("n", "<leader>fb", builtin.buffers, {})
  vim.keymap.set("n", "<leader>fh", builtin.help_tags, {})

end

return M

