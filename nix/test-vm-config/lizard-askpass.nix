# This is based heavily on nixpkgs' nixos/modules/system/boot/unl0kr.nix at commit 66e4c21
{ pkgs, config, lib, outputs, ... }:
let
  cfg = config.boot.initrd.lizard-askpass;
in
{
  options.boot.initrd.lizard-askpass = {
    enable = lib.mkEnableOption "lizard-askpass in initrd" // {
      description = ''Whether to enable lizard-askpass on-screen keyboard in initrd to unlock LUKS.'';
    };

    allowVendorDrivers = lib.mkEnableOption "load optional drivers" // {
      description = ''Whether to load additional drivers for certain vendors (I.E: Wacom, Intel, etc.)'';
    };
  };

  config = lib.mkIf cfg.enable {
    assertions = [
      {
        assertion = cfg.enable -> config.boot.initrd.systemd.enable;
        message = "boot.initrd.lizard-askpass is only supported with boot.initrd.systemd.";
      }
    ];

    boot.initrd.availableKernelModules =
      lib.optionals cfg.enable [
        "hid-generic"
        "usbhid"

        "i2c-designware-core"
        "i2c-designware-platform"
        "i2c-hid-acpi"

        "evdev"
      ]
      ++ lib.optionals cfg.allowVendorDrivers [
        "intel_lpss_pci"
        "elo"
        "wacom"
      ];

    boot.initrd.systemd = {
      storePaths = with pkgs; [
        libinput
        xkeyboard_config
        "${outputs.packages."${config.system}".default}/bin/lizard-askpass"
      ];

      packages = [
        outputs.packages."${config.system}".default
      ];
      users.root.shell = "${outputs.packages."${config.system}".default}/bin/lizard-askpass";

      paths.lizard-askpass.wantedBy = [ "local-fs-pre.target" ];
    };
  };
}

