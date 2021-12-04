{
  description = "Rust Solutions to Advent of Code";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    master.url = "nixpkgs/master";
    nixpkgs.url = "nixpkgs/nixos-21.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
    rust-overlay.inputs.nixpkgs.follows = "unstable";
    unstable.url = "nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachSystem [ "x86_64-darwin" "x86_64-linux" ] (system:
      let
        master = import inputs.master { inherit system; };
        unstable = import inputs.unstable { inherit system; };
        pkgs = import nixpkgs {
          inherit system;

          overlays = [
            inputs.rust-overlay.overlay
            (final: prev: { inherit master unstable; })
          ];
        };
        sharedInputs = with pkgs;
          [
            cargo-edit
            cargo-expand
            cargo-flamegraph
            cargo-sweep
            cargo-watch
            cargo-whatfeatures
            clang
            master.git-cliff
            lld
          ] ++ (with self.packages."${system}"; [ cargo-aoc rust-analyzer ])
          ++ pkgs.lib.optionals (pkgs.stdenv.isDarwin)
          (with pkgs.darwin.apple_sdk.frameworks; [
            CoreServices
            Security
            SystemConfiguration
          ]) ++ lib.optionals (stdenv.isLinux) [ perf-tools strace valgrind ];
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = sharedInputs
            ++ [ pkgs.rust-bin.stable.latest.default ];

          NIX_PATH =
            "nixpkgs=${nixpkgs}:unstable=${inputs.unstable}:master=${inputs.master}";
          RUSTC_WRAPPER = "${pkgs.unstable.sccache}/bin/sccache";
        };

        packages = {
          cargo-aoc = let
            pname = "cargo-aoc";
            version = "0.3.2";
          in pkgs.rustPlatform.buildRustPackage {
            inherit pname version;

            src = pkgs.fetchCrate {
              inherit pname version;
              sha256 = "sha256-4XgaYPfywFBRuuKZoQBl2uifAWvwfeneN1gwCa2vVaQ=";
            };

            cargoSha256 = "sha256-EOS61yuMzFloNOojd5DaVWclcHeF631P7guvbfx6RE0=";

            nativeBuildInputs = [ pkgs.pkg-config ];
            buildInputs = [ pkgs.openssl.dev ];
          };

          gcroot = pkgs.linkFarmFromDrvs "advent"
            (with self.outputs; [ devShell."${system}".inputDerivation ]);

          nightlyDevShell = pkgs.mkShell {
            nativeBuildInputs = sharedInputs
              ++ [ pkgs.rust-bin.nightly.latest.default ];
            RUSTFLAGS = "-Z macro-backtrace";
          };

          rust-analyzer = pkgs.master.rust-analyzer;
        };
      });
}
