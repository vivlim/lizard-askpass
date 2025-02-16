{ pkgs, ... }:
{
  system.stateVersion = "25.05";
  boot.initrd.systemd.enable = true;
  boot.initrd.lizard-askpass.enable = true;
  security.sudo.extraRules = [
    { groups = [ "wheel" ]; commands = [{ command = "ALL"; options = [ "NOPASSWD" ]; }]; }
  ];
  services.openssh.enable = true;
}
