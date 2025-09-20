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
          (python3.withPackages (p: with p; [ gpgme ]))
          mypy
          black
        ];
      };
    };
}
