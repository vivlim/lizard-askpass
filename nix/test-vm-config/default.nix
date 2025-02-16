{ pkgs, ... }:
{
  system.stateVersion = "25.05";
  boot.initrd.systemd.enable = true;
  boot.initrd.lizard-askpass.enable = true;
}
