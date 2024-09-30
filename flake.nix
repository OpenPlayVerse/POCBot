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
      packageFun = import ./Cargo.nix;
    };
    # The workspace defines a development shell with all of the dependencies
    # and environment settings necessary for a regular `cargo build`.
    # Passes through all arguments to pkgs.mkShell for adding supplemental
    # dependencies.
    workspaceShell = rustPkgs.workspaceShell {
      # packages = [ pkgs.somethingExtra ];
      # shellHook = ''
      #   export PS1="\033[0;31m☠dev-shell☠ $ \033[0m"
      # '';
    }; # supports override & overrideAttrs
  in rec {
    packages = {
      pocbot = rustPkgs.workspace.pocbot."1.0.2" {};
      default = packages.pocbot;
    };
    devshell.default = workspaceShell;
  };
}
