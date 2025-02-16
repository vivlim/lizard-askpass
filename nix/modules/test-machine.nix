{ inputs, ... }:
{
  perSystem = { config, self', pkgs, lib, system, nixos-generators, ... }:
    let
      disk-password = "disko"; # this is the default used by disko, i'm not sure how to easily change it
      username = "viv";
      password = "secret";
      test-vm-system =
        inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ../test-vm-config/default.nix
            ../test-vm-config/disk.nix
            ../test-vm-config/lizard-askpass.nix
            {
              users.users."${username}" = {
                isNormalUser = true;
                extraGroups = [ "wheel" ];
                initialPassword = password;
              };
              users.groups.viv = {
                members = [ username ];
              };
            }
            inputs.disko.nixosModules.disko
          ];
          specialArgs = { outputs = self'."${system}".packages; };
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
