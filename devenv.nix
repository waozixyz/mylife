{ pkgs, ... }:

{
  packages = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
    pkg-config
    openssl
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
    # WASM and Trunk-specific tools
    rustup
    trunk
    wasm-bindgen-cli
    lld
  ];

  env = {
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      wayland
      libxkbcommon
      vulkan-loader
      libGL
      libglvnd
    ];
    RUST_BACKTRACE = "1";
  };
  

  enterShell = ''
    echo "Rust WASM development environment with Trunk ready!"
    echo "You can now use 'cargo' to build and 'trunk' to serve your project."
    echo "For WASM development, use 'rustup target add wasm32-unknown-unknown' to add the WASM target."
    echo "Then use 'trunk serve' to build and serve your WASM project."

    # Ensure lld is in the PATH
    export PATH="${pkgs.lld}/bin:$PATH"

    # Verify WASM target is installed
    rustup target list --installed | grep wasm32-unknown-unknown || rustup target add wasm32-unknown-unknown
  '';
}
