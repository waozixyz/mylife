# My Life Timeline

A tool for visualizing life events on a timeline, available in Tcl/Tk (desktop) and JavaScript (web) versions.

## Features

- Display life periods on a timeline
- Customize period names, start dates, and colors
- Support for YAML configuration
- Dynamic updates based on configuration changes

## Development Setup

This project uses `devenv` to manage dependencies. The following packages are included:

- `tcl`, `tk`, `tcllib`
- `libyaml`
- `gcc`
- `raylib` (version `3.7.0`)
- `cmake`, `pkg-config`
- `python3`

### Setup Instructions

1. Install [devenv](https://devenv.sh/).
2. Run `devenv shell` to enter the development environment.

### Commands

- `build-run-c`: Build and run the C version.
- `run-tcl`: Run the Tcl version.
- `serve-web`: Serve the web version (accessible at http://localhost:8000).

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

## Tcl/Tk Version

### Requirements

- Tcl/Tk
- `yaml` package

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

## Raylib Version
### Usage
This project uses raylib for graphical rendering. Ensure consistency by using the specified version in the devenv.nix file.

Use the command to provided in the devenv file to build and run the project.

```
build-run-c
```

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
