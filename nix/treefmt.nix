{ pkgs, ... }:
{
  projectRootFile = "flake.nix";
  settings.excludes = [
    ".gitignore"
    ".gitlint"
    "LICENSE"
    "Cargo.toml"
    "*.slint" # no formatter in treefmt-nix
  ];
  programs = {
    rustfmt.enable = true;
    rustfmt.package = pkgs.rust-bin.nightly.latest.rustfmt;
    nixfmt.enable = true;
    yamlfmt.enable = true;
    mdformat.enable = true;
  };
}
