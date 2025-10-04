{ lib, config, ... }:
let
  cfg = config.modules.boot.loader;
  inherit (lib)
    mkEnableOption
    mkIf
    mkMerge
    mkForce
    ;
  #inherit (lib.types) enum;
in
{
  options = {
    modules.boot.loader = {
      enable = mkEnableOption ''
        NixOS bootloader
      '';
      isSecured = mkEnableOption ''
        Whether to enable SecureBoot
      '';
      /*
        Disabled because grub's SecureBoot support is really bad
        ==TODO==: implement SB for grub
        type = mkOption {
          type = enum [
            "systemd-boot"
            "grub"
          ];
          default = "systemd-boot";
          description = ''
            Type of bootloader
          '';
      */
    };
  };

  config = mkIf cfg.enable (mkMerge [
    {
      boot.loader.efi = {
        canTouchEfiVariables = true;
        efiSysMountPoint = config.disko.devices.disk.nvme.content.partitions.esp.content.mountpoint;
      };
      boot.loader.timeout = 0;
    }

    # mkIf cfg.type == "systemd-boot"
    {
      boot.loader.systemd-boot = {
        enable = mkForce (!cfg.isSecured);
        configurationLimit = 20;
        consoleMode = "max";
        editor = false;
        edk2-uefi-shell.enable = true;
        memtest86.enable = true;
        netbootxyz.enable = true;
      };
    }

    (mkIf cfg.isSecured {
      boot.lanzaboote = {
        enable = true;
        pkiBundle = "/var/lib/sbctl";
      };
    })
  ]);
}
