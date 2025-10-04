{ buildNeovimConfiguration, pkgs }:
(buildNeovimConfiguration {
  inherit pkgs;
  modules =
    let
      c = ../configuration;
    in
    [
      (c + "/languages/python.nix")
      (c + "/completion.nix")
      (c + "/theme.nix")
    ];
}).neovim
