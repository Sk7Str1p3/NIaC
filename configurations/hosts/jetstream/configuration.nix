{
  modules.boot.loader = {
    enable = true;
    isSecured = true;
  };

  nixpkgs.hostPlatform = "x86_64-linux";
  imports = [
    ./hardware/disks.nix
  ];
  users.users.root.password = "root";
}
