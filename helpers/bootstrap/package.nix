{
  python3Packages,
  makeWrapper,
  fetchFromSourcehut,
  disko,
  sops,
  age,
  lib,
  sbctl,
  self,
}:
with python3Packages;
buildPythonApplication {
  pname = "niac_bootstrap";
  version = "0.1.0";
  pyproject = true;
  src = ../.;

  build-system = [
    setuptools
  ];
  dependencies = [
    rich
    gpgme
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
  ];

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram $out/bin/bootstrap \
      --prefix PATH : ${
        lib.makeBinPath [
          disko
          sops
          (sbctl.override {
            databasePath = "/tmp/pki";
          })
        ]
      } \
      --set SELF "${self}"
  '';

  meta.mainProgram = "bootstrap";
}
