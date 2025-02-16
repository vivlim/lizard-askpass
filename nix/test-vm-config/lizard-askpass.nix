# This is based heavily on nixpkgs' nixos/modules/system/boot/unl0kr.nix at commit 66e4c21
{ pkgs, config, lib, inputs, ... }:
let
  cfg = config.boot.initrd.lizard-askpass;
  selfPackages = config.lizard-flake.flake-inputs.self.packages."${pkgs.stdenv.hostPlatform.system}";

  agentServiceUnit = {
    enable = true;
    unitConfig = {
      Description = "Lizard (agent)";
      DefaultDependencies = "no";
      #Conflicts="emergency.service shutdown.target initrd-switch-root.target";
      Conflicts = "systemd-ask-password-console.path systemd-ask-password-console.service";

    };
    serviceConfig = {
      ExecStart = "${selfPackages.lizard_password_agent}/bin/lizard_password_agent";
      SystemCallArchitectures = "native";
      Restart = "on-failure";
      #StandardInput="tty-force";
      #StandardOutput="inherit";
      #StandardError="inherit";

    };
    wants = [
      #"systemd-vconsole-setup.service"
    ];
    after = [
      #"plymouth-start.service"
      "systemd-vconsole-setup.service"
    ];
    wantedBy = [
      "cryptsetup-pre.target"
      "cryptsetup.target"
      "paths.target"
      #"systemd-vconsole-setup.service"
      #"multi-user.target"
    ];
    before = [
      "paths.target"
      "cryptsetup.target"
    ];
  };
  agentPathUnit = {
    enable = true;
    unitConfig = {
      Description = "Lizard (path unit)";
      #Before="systemd-ask-password-console.path";
      After = "plymouth-start.service";
      #Conflicts="getty@tty1.service emergency.service shutdown.target initrd-switch-root.target";
      #Conflicts="systemd-ask-password-console.path";
    };
    pathConfig = {
      DirectoryNotEmpty = "/run/systemd/ask-password";
      MakeDirectory = "yes";
    };
    wantedBy = [
      "cryptsetup-pre.target"
      #"multi-user.target"
    ];
    before = [
      "cryptsetup-pre.target"
    ];
    #    before=[
    #      "emergency.service"
    #      "paths.target"
    #      "cryptsetup.target"
    #      "shutdown.target"
    #    ];
  };
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
      emergencyAccess = true;
      storePaths = with pkgs; [
        libinput
        xkeyboard_config
        selfPackages.lizard_password_agent
        "${selfPackages.lizard_password_agent}/bin/lizard_password_agent"
      ];

      initrdBin = [
        selfPackages.lizard_password_agent
      ];

      #paths.lizard_password_agent = agentPathUnit;
      services.lizard_password_agent = agentServiceUnit;
      #paths.systemd-ask-password-console.enable = false;
      #paths.systemd-ask-password-wall.enable = false;
    };

    #systemd.paths.lizard_password_agent = agentPathUnit;
    systemd.services.lizard_password_agent = agentServiceUnit;
    #systemd.paths.systemd-ask-password-console.enable = false;
    #systemd.paths.systemd-ask-password-wall.enable = false;
    environment.systemPackages = [
      selfPackages.lizard_askpass
      selfPackages.lizard_password_agent
    ];
  };
}

