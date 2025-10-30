{ lib, ... }:
let
  inherit (lib.custom.disks) mkDisk mkPartition;
in
{
  disko.devices.disk = lib.mkMerge [
    (mkDisk {
      name = "nvme";
      device = "nvme-KINGSTON_SKC3000S1024G_50026B7382BF814E";
      partitions = [
        (mkPartition {
          name = "esp";
          size = "512M";
          type = "efi";
          mountpoint = "/boot";
        })
        (mkPartition {
          name = "nixos";
          size = "192G";
          type = "root";
        })
        (mkPartition {
          name = "raid";
          size = "100%";
          type = "raid";
          pairs = [ "/dev/disk/by-partlabel/disk-ssd-raid" ];
          mountpoint = "/media/gameLib";
        })
      ];
    })
    (mkDisk {
      name = "ssd";
      device = "ata-ADATA_SU650_1M1520100693";
      partitions = [
        (mkPartition {
          name = "raid";
          size = "100%";
          type = "raidPair";
        })
      ];
    })
    (mkDisk {
      name = "hdd";
      device = "ata-WDC_WD7500AAKS-00RBA0_WD-WCAPT0571131";
      partitions = [
        (mkPartition {
          name = "hdd";
          size = "100%";
          type = "generic";
          mountpoint = "/media/hdd";
        })
      ];
    })
  ];
}
