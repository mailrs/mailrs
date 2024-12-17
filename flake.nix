{
  description = "The mailrs project";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    unstable-nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils. url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs: inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ]
    (system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (_: _: { } // inputs.self.packages."${system}")
            (import inputs.rust-overlay)
          ];
        };

        callPackage = pkgs.lib.callPackageWith (pkgs // {
          inherit
            callPackage
            buildInputs
            craneLib
            src
            version;
        });

        unstable = import inputs.unstable-nixpkgs {
          inherit system;
        };

        nightlyRustTarget = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          pkgs.rust-bin.fromRustupToolchain { channel = "nightly-2024-12-16"; components = [ "rustfmt" ]; });

        nightlyCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain nightlyRustTarget;

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) version;

        pname = "mailrs";

        src =
          let
            nixFilter = path: _type: !pkgs.lib.hasSuffix ".nix" path;
            extraFiles = path: _type: !(builtins.any (n: pkgs.lib.hasSuffix n path) [ ".github" ".sh" ]);
            filterPath = path: type: builtins.all (f: f path type) [
              nixFilter
              extraFiles
              pkgs.lib.cleanSourceFilter
            ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
          };

        buildInputs = [];

        cargoArtifacts = craneLib.buildDepsOnly {
          inherit src pname buildInputs;
        };

        mailrs = craneLib.buildPackage {
          inherit cargoArtifacts src pname version buildInputs;
          cargoExtraArgs = "--all-features -p mailrs";
        };

        rustfmt' = pkgs.writeShellScriptBin "rustfmt" ''
          exec "${nightlyRustTarget}/bin/rustfmt" "$@"
        '';

        customCargoMultiplexer = pkgs.writeShellScriptBin "cargo" ''
          case "$1" in
            +nightly)
              shift
              export PATH="${nightlyRustTarget}/bin/:''$PATH"
              exec ${nightlyRustTarget}/bin/cargo "$@"
              ;;
            *)
              exec ${rustTarget}/bin/cargo "$@"
          esac
        '';
      in
      rec {
        checks = {
          inherit mailrs;

          mailrs-clippy = craneLib.cargoClippy {
            inherit cargoArtifacts src pname;
            cargoClippyExtraArgs = "--benches --examples --tests --all-features -- --deny warnings";
          };

          mailrs-clippy-no-features = craneLib.cargoClippy {
            inherit cargoArtifacts src pname;
            cargoClippyExtraArgs = "--benches --examples --tests --no-default-features -- --deny warnings";
          };

          mailrs-fmt = nightlyCraneLib.cargoFmt {
            inherit src pname;
          };

          mailrs-tests = craneLib.cargoNextest {
            inherit cargoArtifacts src pname buildInputs;
            nativeBuildInputs = [
              pkgs.coreutils
            ];
          };
        };

        packages = {
          default = packages.mailrs;
          inherit mailrs;
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            customCargoMultiplexer
            rustfmt'
            rustTarget

            pkgs.cargo-insta
            pkgs.cargo-deny
            pkgs.gitlint
            pkgs.statix
          ];
        };
      }
    );
}
