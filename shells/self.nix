{
  perSystem =
    { pkgs, ... }:
    {
    devShells.default = pkgs.mkShellNoCC {
      name = "nix";
      packages = with pkgs; [
          gnupg
          sops
          age
        nixd
        nixfmt
      ];
    };
  };
}
