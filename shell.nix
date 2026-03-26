{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "bakong-khqr";

  buildInputs = with pkgs; [
    rustc
    cargo
    pkg-config
    openssl
  ];

  RUST_BACKTRACE = "1";

  shellHook = ''
    echo "Bakong KHQR Rust SDK Development Environment"
    echo "==============================================="
    echo "Rust version: $(rustc --version)"
    echo "Cargo version: $(cargo --version)"
    echo ""
  '';

  env = {
    OPENSSL_DIR = "${pkgs.openssl.dev}";
    OPENSSL_LIB_DIR = "${pkgs.openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${pkgs.openssl.dev}/include";
  };
}