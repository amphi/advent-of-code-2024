{
  description = "Advent of Code 2024";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    systems.url = "github:nix-systems/x86_64-linux";
    flake-utils.url = "github:numtide/flake-utils";
    flake-utils.inputs.systems.follows = "systems";
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem ( system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        mkRustApp =
          name:
          let
            cargoToml = with builtins; fromTOML (readFile (./. + "/${name}/Cargo.toml"));
          in
          pkgs.rustPlatform.buildRustPackage {
            pname = cargoToml.package.name;
            inherit (cargoToml.package) version;

            src = pkgs.lib.sourceFilesBySuffices (./. + "/${name}") [
              ".rs"
              ".toml"
              ".lock"
            ];

            cargoLock.lockFile = ./. + "/${name}/Cargo.lock";
          };

        mkClippyCheck =
          name:
          (mkRustApp name).overrideAttrs (
            _: prev: {
              pname = prev.pname + "-clippy";
              nativeCheckInputs = (prev.nativeCheckInputs or [ ]) ++ [ pkgs.clippy ];
              checkPhase = "cargo clippy";
            }
          );

        mkRustfmtCheck =
          name:
          (mkRustApp name).overrideAttrs (
            _: prev: {
              pname = prev.pname + "-rustfmt";
              nativeCheckInputs = (prev.nativeCheckInputs or [ ]) ++ [ pkgs.rustfmt ];
              checkPhase = "cargo fmt --check";
            }
          );

        mkTargets =
          names: with pkgs.lib; {
            packages = listToAttrs (
              map (name:
                { name = "${name}";
                  value = mkRustApp name; }
              ) names
            );
            checks = listToAttrs ( lists.flatten (
                map (name: [
                  { name = "${name}-clippy";
                    value = mkClippyCheck name; }
                  { name = "${name}-rustfmt";
                    value = mkRustfmtCheck name; }
                ]) names
              )
            );
          };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            clippy
            rustfmt
          ];

          inputsFrom = builtins.attrValues self.packages.${system};
        };
      }
      // mkTargets [
        "day1"
        "day2"
        "day3"
      ]
    );
}
