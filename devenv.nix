{ pkgs, ... }:

let
  openssl = pkgs.openssl;
in
{
  # Enable Rust language support
  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
  };

  # Packages to be installed in the environment
  packages = with pkgs; [
    appimage-run
    atk
    cairo
    cargo
    cmake
    curl
    file
    fuse
    gcc
    gdk-pixbuf
    glib
    glibc
    glibc.dev
    gtk3
    libayatana-appindicator
    libgcc
    libGL
    libglvnd
    librsvg
    libsoup
    libxkbcommon
    lld
    llvmPackages.bintools
    openssl
    openssl.dev
    pango
    perl
    pkg-config
    rust-analyzer
    rustc
    rustfmt
    rustup
    stdenv.cc.cc
    trunk
    vulkan-headers
    vulkan-loader
    vulkan-tools
    wasm-bindgen-cli
    wayland
    wayland-protocols
    webkitgtk
    wget
    xdotool
  ];

  # Environment variables
  env = {
    OPENSSL_DIR = "${openssl.dev}";
    OPENSSL_LIB_DIR = "${openssl.out}/lib";
    OPENSSL_INCLUDE_DIR = "${openssl.dev}/include";
    CARGO_TERM_COLOR = "always";
    LD_LIBRARY_PATH = with pkgs; lib.makeLibraryPath [
      atk
      cairo
      fuse
      gdk-pixbuf
      glib
      gtk3
      libGL
      libglvnd
      librsvg
      libsoup
      libxkbcommon
      pango
      vulkan-loader
      wayland
      webkitgtk
    ];
    PKG_CONFIG_PATH = with pkgs; lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
      webkitgtk
    ];
    RUST_BACKTRACE = "1";
    RUST_LOG = "debug";
  };

  # Shell hook to run when entering the environment
  enterShell = ''
    echo "Rust WASM development environment with Dioxus and Tauri ready!"
    echo "You can now use 'cargo' to build your project."
    echo "For WASM development, use 'rustup target add wasm32-unknown-unknown' to add the WASM target."
    echo "For AppImage creation, use 'linuxdeploy' (you'll need to download it separately)."
    export PATH="${pkgs.lld}/bin:$PATH"
    rustup target list --installed | grep wasm32-unknown-unknown || rustup target add wasm32-unknown-unknown

    echo "Building dioxus-cli..."
    OPENSSL_DIR=${openssl.dev} \
    OPENSSL_LIB_DIR=${openssl.out}/lib \
    OPENSSL_INCLUDE_DIR=${openssl.dev}/include \
    cargo install dioxus-cli --version 0.5.6 --verbose

    # Fallback to pre-built binary if cargo install fails
    if [ $? -ne 0 ]; then
      echo "Cargo install failed. Downloading pre-built dioxus-cli..."
      mkdir -p $HOME/.local/bin
      curl -L https://github.com/DioxusLabs/dioxus/releases/download/v0.5.6/dioxus_cli-x86_64-unknown-linux-gnu.tar.gz | tar xz -C $HOME/.local/bin
      chmod +x $HOME/.local/bin/dioxus
      export PATH=$HOME/.local/bin:$PATH
    fi
  '';
}