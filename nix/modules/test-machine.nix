{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, system, nixos-generators, ... }:
    let
      disk-password = "disko"; # this is the default used by disko, i'm not sure how to easily change it
      username = "viv";
      password = "secret";
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
      test-vm-system =
        inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ../test-vm-config/default.nix
            ../test-vm-config/disk.nix
            ../test-vm-config/lizard-askpass.nix
            depInject
            {
              users.users."${username}" = {
                isNormalUser = true;
                extraGroups = [ "wheel" ];
                initialPassword = password;
                openssh.authorizedKeys.keys = [
                  "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILd44HChzsbUgnTA+AX86jnzneCq7eKCZgVXqgK7JGSE vivlim@vivdeck"
                ];
              };
              users.groups.viv = {
                members = [ username ];
              };
              services.getty.autologinUser = username;
              #nixpkgs.overlays = [inputs.self.overlays.default];
            }

            inputs.disko.nixosModules.disko
          ];
          specialArgs = { selfPackages = self'.packages; };
        };

      run-test-vm-script = pkgs.writeShellScriptBin "run-test-vm" ''
        echo
        echo 'disk encryption password: ${disk-password}'
        echo 'username: ${username}'
        echo 'password: ${password}'
        echo
        echo 'starting vm...'
        ${test-vm-system.config.system.build.vmWithDisko}/bin/disko-vm
      '';
    in
    {
      packages.run-vm = run-test-vm-script;
      apps.vm = {
        type = "app";
        program = "${run-test-vm-script}/bin/run-test-vm";
      };
    };
}
