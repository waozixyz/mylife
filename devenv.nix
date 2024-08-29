{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown" "aarch64-linux-android" ];
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };
  packages = with pkgs; [
    cargo
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
    # X11 libraries
    xorg.libX11
    xorg.libXi
    # WASM and Trunk-specific tools
    rustup
    trunk
    wasm-bindgen-cli
    wabt
    lld
  ];
  env = {
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      wayland
      libxkbcommon
      vulkan-loader
      libGL
      libglvnd
      xorg.libX11
      xorg.libXi
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
