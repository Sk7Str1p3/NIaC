{
  description = "Sk7Str1p3's NixOS Infrastructure-as-a-Code (NIaC (Nyac <3))";

  # The `flake.lock` generation is not computable[1].
  # So far it's impossible to
  # add helper functions to manage inputs.
  # ... But maybe we can work around this issue with git hooks
  # and shell scripts?
  #
  # [1]: https://github.com/NixOS/nix/issues/5373
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-parts.inputs.nixpkgs-lib.follows = "nixpkgs";

    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    sops.url = "github:Mic92/sops-nix";
    sops.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [
        "x86_64-linux"
      ];

      imports = [
        ./packages/flakeMod.nix
        ./shells/self.nix
      ];
    };
}
