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
          (python3.withPackages (
            p: with p; [
              gpgme
              rich
              (buildPythonPackage rec {
                pname = "sopsy";
                version = "1.2.1";
                pyproject = true;
                src = fetchFromSourcehut {
                  owner = "~nka";
                  repo = "sopsy";
                  rev = version;
                  hash = "sha256-ZFNPOuz0TuPXw8l2zeeWDu7QbDyyabIbH7F/7TfIZmI=";
                };
                build-system = [ hatchling ];
                dependencies = [
                  pyyaml
                ];
              })
            ]
          ))
          mypy
          black
        ];
      };
    };
}
