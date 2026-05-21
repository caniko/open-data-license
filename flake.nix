{
  description = "Open data license enum with SPDX IDs and compatibility rules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";

    flake-utils.inputs.systems.follows = "systems";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        crate = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
        docs = pkgs.stdenv.mkDerivation {
          pname = "open-data-license-docs";
          version = "0.1.0";
          src = pkgs.lib.fileset.toSource {
            root = ./.;
            fileset = pkgs.lib.fileset.maybeMissing ./docs;
          };
          nativeBuildInputs = [pkgs.mdbook];
          phases = ["buildPhase" "installPhase"];
          buildPhase = ''
            cp -r --no-preserve=mode $src/docs docs
            mdbook build docs
          '';
          installPhase = ''
            cp -r docs/book $out
          '';
        };
      in {
        packages = {
          default = crate;
          docs = docs;
          site = docs;
        };

        checks = {
          default = crate;
          fmt = craneLib.cargoFmt {inherit src;};
          clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets --all-features -- --deny warnings";
            });
          test = craneLib.cargoTest (commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--all-features";
            });
          doc = craneLib.mkCargoDerivation (commonArgs
            // {
              pname = "open-data-license-doc";
              cargoArtifacts = null;
              buildPhaseCargoCommand = "RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features";
              installPhaseCommand = ''
                mkdir -p $out
                cp -r target/doc $out/doc
              '';
            });
          docs = docs;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rustToolchain
            pkgs.cargo-audit
            pkgs.cargo-deny
            pkgs.mdbook
          ];
        };
      }
    );
}
