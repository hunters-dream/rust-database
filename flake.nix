{
  description = "Rust Database Project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [
            rustToolchain
            pkgs.pkg-config
            pkgs.clang
            pkgs.lld
          ];

          buildInputs = [
            pkgs.openssl
          ] ++ pkgs.lib.optionals pkgs.stdenv.isLinux [ pkgs.systemdLibs ];

          env = {
            RUSTUP_HOME = "/tmp/nix-rustup-home";
            RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
            RUSTFLAGS = "-C linker=clang -C link-arg=-fuse-ld=lld";
          };

          shellHook = ''
            mkdir -p "$RUSTUP_HOME"
            echo "Rust: $(rustc --version)"
            echo "Cargo: $(cargo --version)"
          '';
        };
      });
}
