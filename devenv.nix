{ pkgs, ... }:

{
  packages = [
    pkgs.tcl
    pkgs.tk
    pkgs.tcllib
    pkgs.libyaml
    pkgs.gcc
    pkgs.raylib
    pkgs.cmake
    pkgs.pkg-config
    pkgs.python3
  ];

  scripts = {
    build-run-c.exec = ''
      cc -o life_c life.c $(pkg-config --cflags --libs yaml-0.1 raylib) -lm -lpthread -ldl -lrt && ./life_c
    '';
    run-tcl.exec = "tclsh main.tcl";
    serve-web.exec = "cd html && python -m http.server 8000";
  };

  env = {
    CFLAGS = "-Wall -Wextra -pedantic -std=c11";
    INCLUDES = builtins.concatStringsSep " " [
      "-I${pkgs.libyaml}/include"
      "-I${pkgs.raylib}/include"
    ];
    LIBS = builtins.concatStringsSep " " [
      "-L${pkgs.libyaml}/lib"
      "-L${pkgs.raylib}/lib"
      "-lyaml"
      "-lraylib"
    ];
    PKG_CONFIG_PATH = "${pkgs.libyaml}/lib/pkgconfig:${pkgs.raylib}/lib/pkgconfig";
  };

  enterShell = ''
    echo "Environment set up. You can use the following commands:"
    echo "  build-run-c  : Build and run the C version"
    echo "  run-tcl      : Run the Tcl version"
    echo "  serve-web    : Serve the web version (accessible at http://localhost:8000)"
    echo ""
  '';
}
