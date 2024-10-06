{
  description = "A flake for building a Rust project with cargo2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cargo2nix.url = "github:cargo2nix/cargo2nix/main";
    flake-utils.follows = "cargo2nix/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    cargo2nix,
    rust-overlay,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [cargo2nix.overlays.default rust-overlay.overlays.default];
      };
      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustVersion = "1.81.0";
      };
      workspaceShell = rustPkgs.workspaceShell {
        # packages = [ pkgs.somethingExtra ];
        # shellHook = ''
        #   export PS1="\033[0;31m☠dev-shell☠ $ \033[0m"
        # '';
      }; # supports override & overrideAttrs
    in rec {
      packages = {
        pocbot = rustPkgs.workspace.pocbot {};
        default = packages.pocbot.bin;
      };
      devShells = {
        default = workspaceShell;
      };
    });
}
