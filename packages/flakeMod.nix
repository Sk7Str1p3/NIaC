{ self, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    {
      packages = {
        bootstrap = pkgs.callPackage ../helpers/bootstrap/package.nix { inherit self; };
      };
    };
}
