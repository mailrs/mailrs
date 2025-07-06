{
  description = "The mailrs project";
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-24.11";
    unstable-nixpkgs.url = "nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs:
    inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (
      system:
      let
        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            (import inputs.rust-overlay)
          ];
        };

        unstable = import inputs.unstable-nixpkgs {
          inherit system;
        };

        nightlyRustTarget = pkgs.rust-bin.selectLatestNightlyWith (
          toolchain:
          pkgs.rust-bin.fromRustupToolchain {
            channel = "nightly-2024-12-16";
            components = [ "rustfmt" ];
          }
        );

        nightlyCraneLib = (inputs.crane.mkLib pkgs).overrideToolchain nightlyRustTarget;

        rustTarget = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (inputs.crane.mkLib pkgs).overrideToolchain rustTarget;

        tomlInfo = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };
        inherit (tomlInfo) version;

        src =
          let
            nixFilter = path: _type: !pkgs.lib.hasSuffix ".nix" path;
            extraFiles =
              path: _type:
              !(builtins.any (n: pkgs.lib.hasSuffix n path) [
                ".github"
                ".sh"
              ]);
            filterPath =
              path: type:
              builtins.all (f: f path type) [
                nixFilter
                extraFiles
                pkgs.lib.cleanSourceFilter
              ];
          in
          pkgs.lib.cleanSourceWith {
            src = ./.;
            filter = filterPath;
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

        treefmt = inputs.treefmt-nix.lib.evalModule pkgs ./nix/treefmt.nix;

        rustSrc = pkgs.lib.fileset.toSource {
          root = ./.;
          fileset =
            let
              includeFilesWithExt = ext: (pkgs.lib.fileset.fileFilter (file: file.hasExt ext) ./.);
            in
            pkgs.lib.fileset.unions (
              [
                ./Cargo.lock
              ]
              ++ (builtins.map includeFilesWithExt [
                "rs"
                "toml"
              ])
            );
        };

        callPackage = pkgs.lib.callPackageWith (
          pkgs
          // {
            inherit
              callPackage
              craneLib
              nightlyCraneLib
              src
              version
              rustSrc
              ;
          }
        );

        crates = pkgs.lib.pipe ./crates [
          builtins.readDir
          builtins.attrNames
          (map (
            name:
            if (builtins.pathExists (./crates + "/${name}/default.nix")) then
              [
                {
                  inherit name;
                  value = callPackage (./crates + "/${name}") { };
                }
              ]
            else
              [ ]
          ))
          builtins.concatLists
          builtins.listToAttrs
        ];
      in
      {
        formatter = treefmt.config.build.wrapper;

        checks =
          let
            individualCrates = pkgs.lib.pipe (builtins.attrNames crates) [
              (map (name: crates."${name}".checks))
            ];
          in
          {
            formatting = treefmt.config.build.check inputs.self;
          }
          // pkgs.lib.attrsets.mergeAttrsList individualCrates;

        packages =
          let
            individualCrates = pkgs.lib.pipe (builtins.attrNames crates) [
              (map (name: crates."${name}".packages))
            ];
          in
          pkgs.lib.attrsets.mergeAttrsList individualCrates;

        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.wayland
            pkgs.libxkbcommon
            pkgs.fontconfig
            pkgs.notmuch
          ];

          nativeBuildInputs = [
            customCargoMultiplexer
            rustfmt'
            rustTarget

            pkgs.notmuch
            pkgs.cargo-insta
            pkgs.cargo-deny
            pkgs.gitlint
            pkgs.statix
          ];
        };
      }
    );
}
