{
  nixpkgs,
  pkgs,
  lib,
  config,
  inputs,
  ...
}: rec {
  languages.rust = {
    channel = "nightly";
    components = [
      "cargo"
      "rust-src"
      "rustc"
    ];
    enable = true;
  };

  packages = [
    pkgs.gcc
    pkgs.openssl
    pkgs.openssl.dev
    pkgs.mold
    pkgs.clang
  ];

  env.LD_LIBRARY_PATH = "${nixpkgs.lib.makeLibraryPath packages}";
}
