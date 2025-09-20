{
  python3Packages,
  makeWrapper,
  disko,
  sops,
  age,
  lib,
  self,
}:
python3Packages.buildPythonApplication {
  pname = "niac_bootstrap";
  version = "0.1.0";
  pyproject = true;
  src = ../.;

  build-system = [
    python3Packages.setuptools
  ];

  nativeBuildInputs = [ makeWrapper ];
  postInstall = ''
    wrapProgram $out/bin/bootstrap \
      --prefix PATH : ${
        lib.makeBinPath [
          disko
          sops
          age
        ]
      } \
      --set SELF "${self}"
  '';

  meta.mainProgram = "bootstrap";
}
