{ pkgs, ... }:

{
  packages = [
    pkgs.tcl
    pkgs.tk
    pkgs.tcllib
    pkgs.gcc
    pkgs.raylib
    pkgs.cmake
    pkgs.pkg-config
    pkgs.python3
  ];

  scripts = {
    build-run-c.exec = "cc -o life_c life.c -lraylib -lm -lpthread -ldl -lrt && ./life_c";
    run-tcl.exec = "tclsh life.tcl";
    serve-web.exec = "cd html && python -m http.server 8000";
  };

  enterShell = ''
    echo "Environment set up. You can use the following commands:"
    echo "  build-run-c  : Build and run the C version"
    echo "  run-tcl      : Run the Tcl version"
    echo "  serve-web    : Serve the web version (accessible at http://localhost:8000)"
 
  '';
}