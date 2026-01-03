{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    # Rust toolchain
    rustc
    cargo
    rustfmt
    clippy
    rust-analyzer

    # Build dependencies
    pkg-config
    openssl

    # Development tools
    cargo-watch
    cargo-edit
    cargo-outdated

    # Additional libraries that are commonly needed
    libiconv
  ] ++ lib.optionals stdenv.isDarwin [
    # macOS specific dependencies
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.CoreServices
    darwin.apple_sdk.frameworks.CoreFoundation
    darwin.apple_sdk.frameworks.Foundation
  ];

  # Environment variables
  RUST_BACKTRACE = "1";

  shellHook = ''
    echo "Rust development environment loaded"
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
  '';
}
