{
  description = "Open data license enum with SPDX IDs and compatibility rules";

  inputs = {
    harbor-rs.url = "git+https://github.com/caniko/harbor-rs.git?ref=trunk&rev=fac8049316846e0ef1c1e6acd92aed7a337b333a";
    rs-harbor.follows = "harbor-rs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    crane.url = "github:ipetkov/crane";
    rust-overlay.url = "github:oxalica/rust-overlay";
    treefmt-nix.url = "github:numtide/treefmt-nix";
    git-hooks.url = "github:cachix/git-hooks.nix";
    simit = {
      url = "git+https://github.com/caniko/simit.git?ref=refs/heads/trunk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    plinth = {
      url = "git+https://codeberg.org/caniko/plinth.git";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };

    flake-utils.inputs.systems.follows = "systems";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    treefmt-nix.inputs.nixpkgs.follows = "nixpkgs";
    git-hooks.inputs.nixpkgs.follows = "nixpkgs";
    systems.url = "github:nix-systems/default";
  };

  outputs = {
    self,
    harbor-rs,
    nixpkgs,
    flake-utils,
    crane,
    rust-overlay,
    treefmt-nix,
    git-hooks,
    simit,
    plinth,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        toolchain = harbor-rs.lib.mkToolchain { inherit pkgs; toolchainProfile = "stable"; };
      rustToolchain = toolchain.rustToolchain;
      craneLib = toolchain.craneLib;
      buildCache = harbor-rs.lib.mkBuildCachePolicy {
        inherit pkgs;
        sccachePackage = harbor-rs.packages.${system}.sccache;
        cacheRoot = null;
        namespaceScope = "canix-rust";
        namespaceGeneration = 5;
      };
        src = craneLib.cleanCargoSource ./.;
        commonArgs = {
          inherit src;
          strictDeps = true;
        };

        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      package = buildCache.withRustCache {
        package = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
      };
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
        website = plinth.lib.${system}.mkProjectSite {
          pname = "open-data-license-website";
          domain = "open-data-license.tartanoglu.com";
          configPath = ./website/plinth-project.toml;
          docsPackage = docs;
        };
      in {
        packages = {
          default = package;
          docs = docs;
          website = website;
          site = website;
        };

        apps.deploy-pages = plinth.lib.${system}.mkDeployPagesApp {
          domain = "open-data-license.tartanoglu.com";
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
              simit.packages.${system}.default
            ]
            ++ pre-commit-check.enabledPackages;
          shellHook = pre-commit-check.shellHook;
        };
      }
    );
}
