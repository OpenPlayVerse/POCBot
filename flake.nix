{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;

      # Common derivation arguments used for all builds
      commonArgs = {
        src = craneLib.cleanCargoSource ./.;
        strictDeps = true;

        buildInputs = with pkgs; [
          # Add extra build inputs here, etc.
          openssl
        ];

        nativeBuildInputs = with pkgs; [
          # Add extra native build inputs here, etc.
          pkg-config
          clang
          mold
        ];
      };

      # Build *just* the cargo dependencies, so we can reuse
      # all of that work (e.g. via cachix) when running in CI
      cargoArtifacts = craneLib.buildDepsOnly (commonArgs
        // {
          # Additional arguments specific to this derivation can be added here.
          # Be warned that using `//` will not do a deep copy of nested
          # structures
          pname = "pocbotcrate-deps";
        });

      # First, run clippy (and deny all warnings) on the crate source.
      pocbotCrateClippy = craneLib.cargoClippy (commonArgs
        // {
          # Again we apply some extra arguments only to this derivation
          # and not every where else. In this case we add some clippy flags
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --allow warnings";
        });

      # Next, we want to run the tests and collect code-coverage, _but only if
      # the clippy checks pass_ so we do not waste any extra cycles.
      pocbotCrateCoverage = craneLib.cargoTarpaulin (commonArgs
        // {
          cargoArtifacts = pocbotCrateClippy;
        });

      # Build the actual crate itself, _but only if the previous tests pass_.
      pocbotCrate = craneLib.buildPackage (commonArgs
        // {
          cargoArtifacts = pocbotCrateCoverage;
        });
    in {
      packages = {
        default = pocbotCrate;
        pocbot = pocbotCrate;
      };
      checks = {
        inherit
          # Build the crate as part of `nix flake check` for convenience
          pocbotCrate
          pocbotCrateCoverage
          ;
      };
    });
}
