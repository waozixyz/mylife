{ pkgs, ... }:

let
  pythonEnv = pkgs.python3.withPackages (ps: with ps; [
    tkinter
    pyyaml
    pip
  ]);
in
{
  packages = [
    pythonEnv
  ];

  scripts = {
    setup-venv = {
      exec = ''
        if [ ! -d .venv ]; then
          ${pythonEnv}/bin/python -m venv .venv
          source .venv/bin/activate
          pip install -r requirements.txt
        else
          echo "Virtual environment already exists."
        fi
      '';
      description = "Set up a Python virtual environment";
    };
    run-python = {
      exec = "python main.py";
      description = "Run the Python version";
    };
  };

  enterShell = ''
    if [ -d .venv ]; then
      source .venv/bin/activate
      export PYTHONPATH="${pythonEnv}/${pythonEnv.sitePackages}:$PYTHONPATH"
    else
      echo "Virtual environment not found. Run 'setup-venv' to create it."
    fi
    echo "Environment set up. You can use the following commands:"
    echo "  setup-venv   : Set up a Python virtual environment"
    echo "  run-python   : Run the Python version"
    echo ""
    if [ ! -d .venv ]; then
      echo "Remember to run setup-venv before run-python if you haven't already."
    fi
  '';
}