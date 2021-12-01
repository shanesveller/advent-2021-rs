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
            # Out-of-order intentional for PATH priority
            self.packages."${system}".rust-analyzer
          ] ++ lib.optionals (stdenv.isLinux) [ perf-tools strace valgrind ];
      in {
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.rust-bin.stable.latest.default ]
            ++ sharedInputs;

          NIX_PATH =
            "nixpkgs=${nixpkgs}:unstable=${inputs.unstable}:master=${inputs.master}";
          RUSTC_WRAPPER = "${pkgs.unstable.sccache}/bin/sccache";
        };

        packages = {
          gcroot = pkgs.linkFarmFromDrvs "advent"
            (with self.outputs; [ devShell."${system}".inputDerivation ]);

          nightlyDevShell = pkgs.mkShell {
            nativeBuildInputs = [ pkgs.rust-bin.nightly.latest.default ]
              ++ sharedInputs;
            RUSTFLAGS = "-Z macro-backtrace";
          };

          rust-analyzer = pkgs.master.rust-analyzer;
        };
      });
}
