{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.tcl
    pkgs.tk
    pkgs.tcllib
  ];

  shellHook = ''
    echo "Environment set up. You can run the application with: tclsh life.tcl"
  '';
}
