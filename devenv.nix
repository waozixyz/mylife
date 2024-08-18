{ pkgs, ... }:

{
  packages = [
    pkgs.python3
    pkgs.python3Packages.tkinter
    pkgs.python3Packages.pyyaml
  ];

  scripts = {
    run-python.exec = "python3 main.py";
  };

  enterShell = ''
    echo "Environment set up. You can use the following commands:"
    echo "  run-python   : Run the Python version"
    echo ""
  '';
}