# My Life Timeline

A Rust-based application for visualizing life events on a timeline, with both native and web versions using egui.

## Features

- Display life periods and life period events on a timeline
- Customize event names, start dates, and colors
- Support for YAML configuration
- Dynamic updates based on configuration changes
- Cross-platform support (native and web)

## Development Setup

This project uses `devenv` to manage dependencies and `trunk` for web builds.

### Prerequisites

- [Nix](https://nixos.org/download.html) with flakes enabled
- [devenv](https://devenv.sh/)

### Setup Instructions

1. Clone the repository:
```
git clone https://github.com/waozixyz/mylife.git
cd mylife
```
2. Enter the development environment:
```
devenv shell
```
3. Build and run the project:
- For native:
```
cargo run
```
- For web:
```
trunk serve
```

## Configuration Format

```yaml
name: John Doe
date_of_birth: 2000-01
life_expectancy: 80
life_periods:
- name: Childhood
 start: 2000-01
 color: "#FFB3BA"
- name: Teenage Years
 start: 2013-01
 color: "#BAFFC9"
 events:
  - name: Winter Internship
    start: 2022-01-03
    color: "#4CAF50"
  - name: Spring Semester
    start: 2022-03-21
    color: "#2196F3"
```

## Native Version
### Requirements
All requirements are managed by devenv and specified in the devenv.nix file.

### Usage
```cargo run```


## Web Version
### Requirements

- Trunk (installed via devenv)
- wasm32-unknown-unknown target (automatically added by devenv)

### Usage
Start the development server:

```trunk serve```

Open the provided URL in a web browser (usually http://127.0.0.1:8080)

## Development
### Adding Dependencies
To add new dependencies, modify the Cargo.toml file and update the devenv.nix file if necessary.

## Building for Release

- Native:
```cargo build --release```

- Web:
```trunk build --release```


## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License. See the LICENSE.md file for details.
