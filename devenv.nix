{ pkgs, ... }:

let
  rust-overlay = import (builtins.fetchTarball {
    url = "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
    sha256 = "sha256:1brxn6ckjgg9mwqrj3cwra3f0m5yyn2hl7j0hp0c1y1h9g1knn3g"; # Replace with the current hash
  });
  pkgs-with-overlay = import pkgs.path {
    overlays = [ rust-overlay ];
    inherit (pkgs) system;
  };
in
{
  packages = with pkgs-with-overlay; [
    openssl
    pkg-config
    eza
    fd
    rust-bin.stable.latest.default
    cmake
    wayland
    wayland-protocols
    libxkbcommon
    libGL
    vulkan-loader
    vulkan-tools
    vulkan-headers
    libglvnd
    llvmPackages.bintools
    trunk
    wasm-bindgen-cli
    lld
  ];

  env = {
    LD_LIBRARY_PATH = with pkgs-with-overlay; lib.makeLibraryPath [
      wayland
      libxkbcommon
      vulkan-loader
      libGL
      libglvnd
      openssl
    ];
    RUST_BACKTRACE = "1";
  };

  enterShell = ''
    echo "Rust $(rustc --version) WASM development environment with Trunk ready!"
    echo "You can now use 'cargo' to build and 'trunk' to serve your project."
    echo "For WASM development, use 'rustup target add wasm32-unknown-unknown' to add the WASM target."
    echo "Then use 'trunk serve' to build and serve your WASM project."

    export PATH="${pkgs-with-overlay.lld}/bin:$PATH"

    rustup target list --installed | grep wasm32-unknown-unknown || rustup target add wasm32-unknown-unknown

    alias ls=eza
    alias find=fd
  '';
}