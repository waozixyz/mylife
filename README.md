# My Life Timeline

A simple application for visualizing life events on a timeline. Available in two versions: Tcl/Tk (desktop) and JavaScript (web).

## Features

- Visualize life periods on a timeline
- Customize period names, start dates, and colors
- YAML configuration file support
- Dynamic updates based on configuration changes

## Tcl/Tk Version

### Requirements

- Tcl/Tk
- `yaml` package

A `shell.nix` file is provided for easy setup using Nix.

### Usage
```
tclsh life.tcl [years] [yaml_file]
```
- `years`: (Optional) Number of years to display (default: 100)
- `yaml_file`: (Optional) Name of the YAML file in the `data` directory to load

## JavaScript Version

### Usage

1. Open the HTML file in a web browser
2. Modify the configuration using the form
3. Load/Save configurations using the provided buttons

## Configuration Format

```yaml
name: John Doe
date_of_birth: "2000-01"
life_periods:
  - name: Childhood
    start: "2000-01"
    color: "#FFB3BA"
  - name: Teenage Years
    start: "2013-01"
    color: "#BAFFC9"
```

Place YAML files in the data directory for the Tcl/Tk version.

## Installation
Tcl/Tk Version

1. Use the provided shell.nix file with Nix, or
2. Install Tcl/Tk and the yaml package manually

## JavaScript Version
Ensure all files (HTML, JS, CSS, js-yaml.min.js) are in the same directory and open the HTML file in a browser.