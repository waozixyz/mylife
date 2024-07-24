{ pkgs ? import <nixpkgs> {} }:

let
  # Custom script to build and run the C version
  buildRunC = pkgs.writeScriptBin "build-run-c" ''
    #!/bin/sh
    cc -o life_c life.c -lraylib -lm -lpthread -ldl -lrt
    ./life_c
  '';

  # Custom script to run the Tcl version
  runTcl = pkgs.writeScriptBin "run-tcl" ''
    #!/bin/sh
    tclsh life.tcl
  '';

  # Custom script to serve the web version
  serveWeb = pkgs.writeScriptBin "serve-web" ''
    #!/bin/sh
    cd html
    ${pkgs.python3}/bin/python -m http.server 8000
  '';

in pkgs.mkShell {
  buildInputs = [
    # Tcl dependencies
    pkgs.tcl
    pkgs.tk
    pkgs.tcllib

    # C and Raylib dependencies
    pkgs.gcc
    pkgs.raylib
    pkgs.cmake
    pkgs.pkg-config

    # Web dependencies
    pkgs.python3

    # Custom scripts
    buildRunC
    runTcl
    serveWeb
  ];

  shellHook = ''
    echo "Environment set up. You can use the following commands:"
    echo "  build-run-c  : Build and run the C version"
    echo "  run-tcl      : Run the Tcl version"
    echo "  serve-web    : Serve the web version (accessible at http://localhost:8000)"
  '';
}