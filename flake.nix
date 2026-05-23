{
  description = "Open data license enum with SPDX IDs and compatibility rules";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    git-hooks.url = "github:cachix/git-hooks.nix";

    flake-utils.inputs.systems.follows = "systems";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    git-hooks.inputs.nixpkgs.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    treefmt-nix,
    git-hooks,
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
        package = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
        treefmtEval = treefmt-nix.lib.evalModule pkgs (import ./nix/treefmt.nix);
        pre-commit-check = git-hooks.lib.${system}.run {
          src = ./.;
          hooks = import ./nix/pre-commit.nix {
            inherit pkgs;
            treefmtWrapper = treefmtEval.config.build.wrapper;
            inherit rustToolchain;
          };
        };
        docs = pkgs.stdenv.mkDerivation {
          pname = "open-data-license-docs";
          version = "0.2.0";
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
          default = package;
          docs = docs;
          site = docs;
        };

        checks = {
          default = package;
          formatting = treefmtEval.config.build.check self;
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

        formatter = treefmtEval.config.build.wrapper;

        devShells.default = pkgs.mkShell {
          packages =
            [
              rustToolchain
              pkgs.cargo-audit
              pkgs.cargo-deny
              pkgs.mdbook
              pkgs.pre-commit
              pkgs.rust-analyzer
            ]
            ++ pre-commit-check.enabledPackages;
          shellHook = pre-commit-check.shellHook;
        };
      }
    );
}
