{ inputs, ... }:
let
  depInject = { pkgs, lib, ... }: {
    options.lizard-flake = lib.mkOption {
      type = with lib.types; attrsOf unspecified;
      default = { };
    };
    config.lizard-flake = {
      # inputs comes from the outer environment of flake.nix
      flake-inputs = inputs;
    };
  };
in
{
  flake = {
    nixosModules.default = { pkgs, lib, ... }: {
      imports = [
        ../test-vm-config/lizard-askpass.nix
        depInject
      ];
    };
  };

  #  imports = [
  #    inputs.flake-parts.flakeModules.easyOverlay
  #  ];
  #  perSystem = {config, pkgs, final, ... }: {
  #    overlayAttrs = {
  #      lizard_askpass = config.packages.lizard_askpass;
  #      lizard_password_agent = config.packages.lizard_password_agent;
  #    };
  #  };

}
