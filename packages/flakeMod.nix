{ self, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    {
      packages = {
        bootstrap = pkgs.callPackage ../helpers/bootstrap_py/package.nix { inherit self; };
      };
    };
}
