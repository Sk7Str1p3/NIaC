{ lib, ... }:
{
  vim.ui.colorful-menu-nvim = {
    enable = true;
  };
  vim.autocomplete.blink-cmp = {
    enable = true;
    friendly-snippets.enable = true;
    mappings = {
      close = "<Esc>";
      complete = null;
      confirm = "<CR>";
      next = "<Tab>";
      previous = "<S-Tab>";
      scrollDocsUp = "<C-k>";
      scrollDocsDown = "<C-j>";
    };
    sourcePlugins = {
      emoji.enable = true;
      ripgrep.enable = true;
      spell.enable = true;
    };
    setupOpts = {
      fuzzy.implementation = "rust";
      completion = {
        ghost_text = {
          enable = true;
          show_without_selection = true;
        };
        documentation.window.border = "rounded";
        menu.border = "rounded";
      };
    };
  };
  vim.diagnostics = {
    enable = true;
    config = {
      virtual_lines = true;
    };
  };
}
