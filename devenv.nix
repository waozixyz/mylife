{ pkgs, ... }:
{
  languages.rust = {
    enable = true;
    channel = "stable";
    targets = [ "x86_64-unknown-linux-gnu" "wasm32-unknown-unknown"];
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
    rustup
    trunk
    wasm-bindgen-cli
    lld
    curl
    wget
    file
    xdotool
    libayatana-appindicator
    librsvg
    gtk3
    webkitgtk
    webkitgtk_4_1
    webkitgtk_4_1.dev
    glib
    cairo
    pango
    atk
    gdk-pixbuf
    librsvg
    libsoup
  ];
  env = {
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      wayland
      libxkbcommon
      vulkan-loader
      libGL
      libglvnd
      gtk3
      webkitgtk
      webkitgtk_4_1
      glib
      cairo
      pango
      atk
      gdk-pixbuf
      librsvg
      libsoup
    ];
    RUST_BACKTRACE = "1";
    PKG_CONFIG_PATH = with pkgs; lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
      webkitgtk_4_1
      # Add other packages that might provide .pc files here
    ];
  };
  
  enterShell = ''
    echo "Rust WASM development environment with Dioxus and Tauri ready!"
    echo "You can now use 'cargo' to build your project."
    echo "For WASM development, use 'rustup target add wasm32-unknown-unknown' to add the WASM target."
    export PATH="${pkgs.lld}/bin:$PATH"
    rustup target list --installed | grep wasm32-unknown-unknown || rustup target add wasm32-unknown-unknown
  '';
}
