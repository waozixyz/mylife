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
    echo "egui development environment for Wayland ready!"
    echo "You can now use 'cargo' to build and run your project."
  '';
}
