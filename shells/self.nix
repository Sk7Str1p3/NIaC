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
          (rust-bin.fromRustupToolchainFile ../rust-toolchain.toml)
          taplo
        ];
      };
    };
}
