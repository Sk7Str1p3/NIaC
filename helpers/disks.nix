# Helper functions for easy disk configuration
{ lib, ... }:
{
  mkDisk =
    {
      name,
      device,
      partitions,
    }:
    {
      ${name} = {
        type = "disk";
        device = "/dev/disk/by-id/" + device;
        content = {
          type = "gpt";
          partitions = lib.mkMerge partitions;
        };
      };
    };
  mkPartition =
    {
      name,
      size,
      isEncrypted ? false,
      mountpoint ? null,
      type,
      pairs ? null, # used if partition is raid
    }:
    let
      types = {
        "raid" = "FD00";
        "efi" = "EF00";
        "generic" = "8300";
        "root" = "8300";
        "raidPair" = "FD00";
      };
      mountOptions = [
        "compress-force=zstd"
        "noatime"
      ];
      structs = {
        root = {
          type = "btrfs";
          extraArgs = [ "-f" ];
          subvolumes = {
            "root" = {
              mountpoint = "/";
              inherit mountOptions;
            };
            "store" = {
              mountpoint = "/nix";
              inherit mountOptions;
            };
            "swap" = {
              mountpoint = "/.swapvol";
              inherit mountOptions;
              swap = {
                _.size = "12G";
              };
            };
          };
        };
        generic = {
          type = "btrfs";
          extraArgs = [ "-f" ];
          subvolumes = {
            "main" = {
              inherit mountpoint mountOptions;
            };
          };
        };
        efi = {
          type = "filesystem";
          format = "vfat";
          inherit mountpoint;
        };
        raid = {
          type = "btrfs";
          extraArgs = [
            "-f"
            "-d raid1 -m raid1"
          ]
          ++ pairs;
          subvolumes = {
            "main" = {
              inherit mountpoint mountOptions;
            };
          };
        };
        raidPair = null;
      };
    in
    assert lib.assertMsg (type == "efi" -> !isEncrypted) "EFI system partition cannot be encrypted!";
    assert lib.assertMsg (
      type == "raid" -> pairs != null
    ) "RAID partition must have a second partition!";
    assert lib.assertMsg (
      type == "raidPair" -> mountpoint == null
    ) "RAID secondary partition should not have a mountpoint!";
    {
      ${name} = {
        inherit size;
        type = types.${type};
        content =
          if isEncrypted then
            {
              type = "luks";
              # TODO: sops-encrypted key files
              # settings = {
              #  keyFile = ...;
              #  passwordFile = ...;
              # };
              content = structs.${type};
            }
          else
            structs.${type};
      };
    };
}
