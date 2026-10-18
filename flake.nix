{
  description = "Product-neutral schema identity and compatibility core";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/20535e48e12c86043b577b8518234ff5dbb26957";
    flake-utils.url = "github:numtide/flake-utils/11707dc2f618dd54ca8739b309ec4fc024de578b";
    rust-overlay = {
      url = "github:oxalica/rust-overlay/3c38e1e1ba9c8d7030f7b5a801398ea7d8a6fdc0";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane/b556d7bbae5ff86e378451511873dfd07e4504cd";
    octet = {
      url = "git+ssh://git@github.com/OnixResearch/octet.git?rev=07b96f6f75ea081d1fe1d7163372af37128b3a91";
      inputs.crane.follows = "crane";
    };
    cairn.url = "git+ssh://git@github.com/OnixResearch/cairn.git?rev=bfa88df5b2ac5f05e4a977bb8d4b8248d4511449";
    nickel-1-17-0.url = "github:nickel-lang/nickel/1320a983e6c3d1e2fb53dd2464b084b4903b1426";
    content-identity-core.url = "git+https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git?rev=7f55597b5dc879b7601856e8d7fd0dbacaa2a498";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      octet,
      cairn,
      nickel-1-17-0,
      content-identity-core,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;
        nickel = nickel-1-17-0.packages.${system}.default;
        cairnCli = cairn.packages.${system}.default;
        cargoSource = pkgs.lib.cleanSourceWith {
          src = self;
          filter =
            path: type:
            craneLib.filterCargoSources path type
            || pkgs.lib.hasInfix "/fixtures/" (toString path)
            || pkgs.lib.hasSuffix "/crates/schema-identity-conformance/src/vectors.json" (toString path);
        };
        cargoVendorDir = craneLib.vendorCargoDeps {
          cargoLock = ./Cargo.lock;
          outputHashes = {
            "git+https://seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git?rev=7f55597b5dc879b7601856e8d7fd0dbacaa2a498#7f55597b5dc879b7601856e8d7fd0dbacaa2a498" =
              "sha256-dwcdRV9LBJEczU9fLN6AYT7cpLcke3auJa7ffPHmoyc=";
          };
        };
        commonArgs = {
          pname = "schema-identity-core";
          version = "0.1.0";
          src = cargoSource;
          inherit cargoVendorDir;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        octetConfig = (builtins.fromJSON (builtins.readFile ./config/octet-deny-all.json)).config;
        octetFlags = builtins.concatStringsSep " " (map (lint: "-D ${lint}") octetConfig.lints);
        octetRunner = pkgs.writeShellApplication {
          name = "octet-deny-all";
          runtimeInputs = [ octet.packages.${system}.cargo-octet ];
          text = ''
            export DYLINT_RUSTFLAGS=${pkgs.lib.escapeShellArg octetFlags}
            exec cargo-octet check "$@"
          '';
        };
        octetCheck =
          (octet.lib.mkConsumerCheck {
            inherit system;
            src = self;
            cargoLock = ./Cargo.lock;
          }).overrideAttrs
            (_old: {
              DYLINT_RUSTFLAGS = octetFlags;
            });
      in
      {
        packages = {
          default = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--workspace --all-targets --all-features";
            }
          );
          octet-deny-all = octetRunner;
        };

        checks = {
          cargo-test = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--workspace --all-targets --all-features";
            }
          );
          cargo-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--workspace --all-targets --all-features -- -D warnings";
            }
          );
          cargo-format = craneLib.cargoFmt {
            pname = "schema-identity-core-format";
            src = cargoSource;
          };
          core-no-std-wasm = craneLib.cargoBuild (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoExtraArgs = "-p schema-identity-core --target wasm32-unknown-unknown";
            }
          );
          fixture-cohort =
            pkgs.runCommand "schema-identity-fixture-cohort"
              {
                nativeBuildInputs = [
                  nickel
                  pkgs.diffutils
                ];
              }
              ''
                    test '${nickel-1-17-0.rev}' = '1320a983e6c3d1e2fb53dd2464b084b4903b1426'
                    test "$(nickel --version)" = 'nickel-lang-cli nickel 1.17.0 (rev 1320a98)'
                    cd ${self}
                    nickel typecheck fixtures/vectors.ncl
                    nickel export --format json fixtures/vectors.ncl > "$TMPDIR/vectors.json"
                    diff --unified fixtures/generated/vectors.json "$TMPDIR/vectors.json"
                diff --unified crates/schema-identity-conformance/src/vectors.json "$TMPDIR/vectors.json"
                    for fixture in fixtures/invalid/*.ncl; do
                      if nickel export --format json "$fixture" > /dev/null 2>&1; then
                        echo "invalid schema identity fixture passed: $fixture" >&2
                        exit 1
                      fi
                    done
                    touch "$out"
              '';
          source-inventory =
            pkgs.runCommand "schema-identity-source-inventory"
              {
                nativeBuildInputs = [ pkgs.ripgrep ];
              }
              ''
                cd ${self}
                rg --fixed-strings 'f722df98343336d95044309639e83283214239eb' SOURCE.md > /dev/null
                rg --fixed-strings '7f55597b5dc879b7601856e8d7fd0dbacaa2a498' SOURCE.md Cargo.toml flake.nix > /dev/null
                test '${content-identity-core.rev}' = '7f55597b5dc879b7601856e8d7fd0dbacaa2a498'
                test "$(rg --count 'seed.radicle.garden/zA14jhTdwST4PxPBdsryc4zS7Kpq.git' Cargo.toml)" = 1
                touch "$out"
              '';
          source-boundary =
            pkgs.runCommand "schema-identity-source-boundary"
              {
                nativeBuildInputs = [
                  nickel
                  pkgs.ripgrep
                ];
              }
              ''
                cd ${self}
                nickel typecheck profiles/source-boundary.ncl
                nickel export --format json profiles/source-boundary.ncl > /dev/null
                for fixture in profiles/fixtures/source-invalid/*.ncl; do
                  if nickel export --format json "$fixture" > /dev/null 2>&1; then
                    echo "invalid source boundary passed: $fixture" >&2
                    exit 1
                  fi
                done
                rg --fixed-strings 'rad:z6gGpUJtzdVBCCtZTzh4cV1skv4H' SOURCE.md > /dev/null
                rg --fixed-strings '2562c8aa38a034061f9af9f3e17280494a5b8de2' SOURCE.md > /dev/null
                touch "$out"
              '';
          content-identity-boundary =
            pkgs.runCommand "schema-identity-content-identity-boundary"
              {
                nativeBuildInputs = [ pkgs.ripgrep ];
              }
              ''
                pattern='SchemaIdentity|schema_alias|MigrationReference|CompatibilityDecision'
                if rg "$pattern" ${content-identity-core}/src; then
                  echo 'content-identity-core contains schema policy' >&2
                  exit 1
                fi
                if ! rg "$pattern" ${self}/fixtures/content-identity-boundary/invalid.rs > /dev/null; then
                  echo 'negative content identity boundary fixture did not trigger' >&2
                  exit 1
                fi
                touch "$out"
              '';
          cairn-lifecycle =
            pkgs.runCommand "schema-identity-cairn-lifecycle"
              {
                nativeBuildInputs = [
                  cairnCli
                  pkgs.ripgrep
                ];
              }
              ''
                cp -R ${self} repo
                chmod -R u+w repo
                cd repo
                cairn policy export --check
                cairn validate --root .
                if test -d .cairn/changes/extract-schema-identity-core; then
                  cairn gate proposal extract-schema-identity-core --root .
                  cairn gate design extract-schema-identity-core --root .
                  cairn gate tasks extract-schema-identity-core --root .
                else
                  set -- .cairn/archive/*-extract-schema-identity-core
                  test -d "$1"
                  if rg '\[ \]' "$1/tasks.md"; then
                    echo 'archived schema identity task remains incomplete' >&2
                    exit 1
                  fi
                fi
                touch "$out"
              '';
          octet-deny-all = octetCheck;
        };

        devShells.default = pkgs.mkShell {
          packages = [
            cairnCli
            nickel
            octetRunner
            pkgs.nixfmt
            pkgs.pre-commit
            rustToolchain
          ];
        };

        formatter = pkgs.nixfmt;
      }
    );
}
