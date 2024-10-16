{
  description = "Rust WASM development environment with Dioxus";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "wasm32-unknown-unknown" "aarch64-linux-android" "armv7-linux-androideabi" "i686-linux-android" "x86_64-linux-android" ];
          extensions = [ "rust-src" "rust-analyzer" "clippy" "rustfmt" ];
        };
      in
      {
        # for github actions devShell
        devShells.custom = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
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
            trunk
            lld
            curl
            wget
            file
            xdotool
            libayatana-appindicator
            librsvg
            gtk3
            webkitgtk
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            libsoup
            appimage-run
            fuse
            gcc
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.vulkan-loader
              pkgs.libGL
              pkgs.libglvnd
              pkgs.gtk3
              pkgs.webkitgtk
              pkgs.glib
              pkgs.cairo
              pkgs.pango
              pkgs.atk
              pkgs.gdk-pixbuf
              pkgs.librsvg
              pkgs.libsoup
              pkgs.fuse
            ]}
            export RUST_BACKTRACE="1"
            export PKG_CONFIG_PATH=${pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
              pkgs.webkitgtk
            ]}
            export PATH="${pkgs.lld}/bin:$PATH"

            echo "Rust WASM development environment with Dioxus ready!"
            echo "You can now use 'cargo' to build your project."
            echo "For WASM development, use 'rustup target add wasm32-unknown-unknown' to add the WASM target."
            echo "For AppImage creation, use 'linuxdeploy' (you'll need to download it separately)."

            # Install latest dioxus-cli
            cargo install --force dioxus-cli

            # Install latest wasm-bindgen-cli
            cargo install --force wasm-bindgen-cli

            # Ensure wasm32 target is installed
            rustup target add wasm32-unknown-unknown
          '';
        };

        # default devShell
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustToolchain
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
            trunk
            lld
            curl
            wget
            file
            xdotool
            libayatana-appindicator
            librsvg
            gtk3
            glib
            cairo
            pango
            atk
            gdk-pixbuf
            libsoup
            appimage-run
            fuse
            gcc
      
            graphite2
            xorg.libXft
            xorg.libXinerama
            cups
            gmp
            webkitgtk_4_1
          ];

          shellHook = ''
            export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath [
              pkgs.wayland
              pkgs.libxkbcommon
              pkgs.vulkan-loader
              pkgs.libGL
              pkgs.libglvnd
              pkgs.gtk3
              pkgs.webkitgtk_4_1
              pkgs.glib
              pkgs.cairo
              pkgs.pango
              pkgs.atk
              pkgs.gdk-pixbuf
              pkgs.librsvg
              pkgs.libsoup
              pkgs.fuse                    
              pkgs.graphite2
              pkgs.xorg.libXft
              pkgs.xorg.libXinerama
              pkgs.cups
              pkgs.gmp
            ]}
            export RUST_BACKTRACE="1"
            export PKG_CONFIG_PATH=${pkgs.lib.makeSearchPathOutput "dev" "lib/pkgconfig" [
              pkgs.webkitgtk_4_1
              pkgs.graphite2
              pkgs.xorg.libXft
              pkgs.xorg.libXinerama
              pkgs.cups
              pkgs.gmp
            ]}
            export PATH="$HOME/.cargo/bin:$PATH"


            echo "Custom Rust WASM development environment with Dioxus ready!"

            if ! command -v dx &> /dev/null; then
              echo "Installing dioxus-cli..."
              cargo install dioxus-cli
            else
              echo "dioxus-cli (dx) is already installed."
            fi

            if ! command -v wasm-bindgen &> /dev/null; then
              echo "Installing wasm-bindgen-cli..."
              cargo install wasm-bindgen-cli
            else
              echo "wasm-bindgen-cli is already installed."
            fi

            # Ensure wasm32 target is installed
            if ! rustup target list --installed | grep -q "wasm32-unknown-unknown"; then
              rustup target add wasm32-unknown-unknown
            fi
          '';
        };
      }
    );
}
