{
  perSystem = {pkgs,...}: {
    devShells.default = pkgs.mkShellNoCC {
      name = "nix";
      packages = with pkgs; [
        nixd
        nixfmt
      ];
    };
  };
}