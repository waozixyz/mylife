# My Life Timeline

A Rust-based application for visualizing life events on a timeline, with both native and web versions using egui.

## Features

- Display life periods and life period events on a timeline
- Customize event names, start dates, and colors
- Support for YAML configuration
- Dynamic updates based on configuration changes
- Cross-platform support (native and web)

## Development Setup

This project uses `devenv` to manage dependencies and `dioxus-cli` for building and testing.

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
3. Run with live reloading:
- For desktop:
```
dx serve --platform desktop
```
- For web:
```
dx serve
```

## Configuration Format
See files in the [data](/data) folder


## Development
### Adding Dependencies
To add new dependencies, modify the Cargo.toml file and update the devenv.nix file if necessary.

## Building for Release

- Native:
```dx build --release --platform desktop```

- Web:
```dx build --release --platform web```


## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
