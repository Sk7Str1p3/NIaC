{ self, inputs, ... }:
{
  perSystem =
    {
      pkgs,
      ...
    }:
    {
      packages = {
        bootstrap = pkgs.callPackage ../helpers/bootstrap_py/package.nix { inherit self; };
        nvim-python = pkgs.callPackage ./nvim/python/package.nix {
          buildNeovimConfiguration = inputs.nvf.lib.neovimConfiguration;
          inherit pkgs;
        };
      };
    };
}
