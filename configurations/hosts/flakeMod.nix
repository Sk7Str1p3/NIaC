{
  lib,
  inputs,
  ...
}:
{
  flake.nixosConfigurations = lib.mkMerge (
    map (host: {
      ${host} = lib.nixosSystem {
        specialArgs = { inherit lib inputs; };
        modules = [
          ./${host}/configuration.nix
          ../../modules/system/flakeMod.nix
          inputs.disko.nixosModules.disko
          inputs.lanzaboote.nixosModules.lanzaboote
        ];
      };
    }) [ "jetstream" ]
  );
}
