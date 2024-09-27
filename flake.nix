{
  description = "A flake for building a Rust project with cargo2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cargo2nix.url = "github:cargo2nix/cargo2nix";
  };

  outputs = {
    self,
    nixpkgs,
    cargo2nix,
  }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      overlays = [cargo2nix.overlays.default];
    };
    rustPkgs = pkgs.rustBuilder.makePackageSet {
      rustVersion = "1.75.0";
      packageFun = import ./Cargo.nix;
    };
  in {
    packages.${system}.default = rustPkgs.workspace.build;
    devShells.${system}.default = rustPkgs.workspace.shell;
  };
}
