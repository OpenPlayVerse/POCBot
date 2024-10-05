{
  description = "A flake for building a Rust project with cargo2nix";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    cargo2nix.url = "github:cargo2nix/cargo2nix/main";
    flake-utils.follows = "cargo2nix/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
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
        overlays = [cargo2nix.overlays.default];
      };
      rustPkgs = pkgs.rustBuilder.makePackageSet {
        packageFun = import ./Cargo.nix;
        rustVersion = "latest";
      };
      # The workspace defines a development shell with all of the dependencies
      # and environment settings necessary for a regular `cargo build`.
      # Passes through all arguments to pkgs.mkShell for adding supplemental
      # dependencies.
      workspaceShell = pkgs.mkShell {
        # shellHook = ''
        #   export PS1="\033[0;31m☠dev-shell☠ $ \033[0m"
        # '';
      };
    in rec {
      packages = {
        pocbot = rustPkgs.workspace.pocbot {};
        default = packages.pocbot;
      };
      devShells = {
        default = workspaceShell;
      };
    });
}
