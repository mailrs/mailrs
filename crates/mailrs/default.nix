{
  craneLib,
  nightlyCraneLib,
  pkgs,
  src,
  version,
  ...
}:

let
  pname = "mailrs";

  buildInputs = [
    pkgs.pkg-config
    pkgs.fontconfig
    pkgs.wayland-protocols
  ];

  nativeBuildInputs = [
    pkgs.notmuch
  ];

  cargoArtifacts = craneLib.buildDepsOnly {
    inherit
      src
      pname
      version
      buildInputs
      ;
  };

  mailrs = craneLib.buildPackage {
    inherit
      cargoArtifacts
      src
      pname
      version
      buildInputs
      nativeBuildInputs
      ;

    cargoExtraArgs = "--all-features -p mailrs";
  };

  mailrs-doc = craneLib.cargoDoc {
    inherit
      cargoArtifacts
      src
      pname
      version
      buildInputs
      ;

    cargoExtraArgs = "--document-private-items -p mailrs";
  };

in
{
  packages = {
    inherit
      mailrs
      mailrs-doc
      ;
  };

  checks = {
    mailrs-clippy = craneLib.cargoClippy {
      inherit
        cargoArtifacts
        src
        pname
        version
        ;
      cargoClippyExtraArgs = "--benches --examples --tests --all-features -- --deny warnings";
    };

    mailrs-clippy-no-gui = craneLib.cargoClippy {
      inherit
        cargoArtifacts
        src
        pname
        version
        ;
      cargoClippyExtraArgs = "--benches --examples --tests --no-default-features --features gui -- --deny warnings";
    };

    mailrs-clippy-no-tui = craneLib.cargoClippy {
      inherit
        cargoArtifacts
        src
        pname
        version
        ;
      cargoClippyExtraArgs = "--benches --examples --tests --no-default-features --features tui -- --deny warnings";
    };

    mailrs-fmt = nightlyCraneLib.cargoFmt {
      inherit src pname version;
    };

    mailrs-tests = craneLib.cargoNextest {
      inherit
        cargoArtifacts
        src
        pname
        buildInputs
        version
        ;

      nativeBuildInputs = nativeBuildInputs ++ [
        pkgs.coreutils
      ];
    };
  };
}
