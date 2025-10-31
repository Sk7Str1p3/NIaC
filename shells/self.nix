{ inputs, ... }:
{
  perSystem =
    { pkgs, ... }:
    let
      pkgs' = pkgs.appendOverlays [ (import inputs.rust-overlay) ];
    in
    {
      devShells.default = pkgs'.mkShellNoCC {
        name = "nix";
        packages = with pkgs'; [
          gnupg
          sops
          age
          nixd
          nixfmt
          (rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
            ];
          })
          taplo
        ];
      };
    };
}
