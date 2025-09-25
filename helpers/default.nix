{ lib, ... }:
{
  disks = import ./disks.nix { inherit lib; };
}
