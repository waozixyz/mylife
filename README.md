# myQuest

A personal life management application focused on visualization and habit tracking.

## Features

### Habit Tracker
Track your daily habits and build consistency:
- Mark completed habits on a calendar view
- Visual progress tracking
- Simple and intuitive interface

### Weekly Todo List
Organize your tasks by day of the week:
- Daily task organization
- Easy task management
- Week-at-a-glance view

### Life Timeline
Visualize your life journey:
- View your life progression on an interactive timeline
- Compare past experiences with future possibilities
- Gain perspective on time allocation
- Understand your life's bigger picture

### Routine Manager (Coming Soon)
Plan and visualize your daily routines:
- Create detailed daily schedules
- Visualize time allocation
- Optimize your daily workflow

## Development

### Prerequisites
- Rust toolchain (install via [rustup](https://rustup.rs/))
- Dioxus CLI

### Getting Started

1. Install the Rust toolchain:

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
2. Install Dioxus CLI:
```
cargo install dioxus-cli
```
3. Clone the repository:
```
git clone https://github.com/waozixyz/myquest.git
cd myquest
```
4. Run the development server:
```
dx serve
```
### Building

Build for different platforms using Dioxus CLI:

- Web:
```
dx build --release --platform web
```
- Desktop:
```
dx build --release --platform desktop
```
- Android:
```
dx build --release --platform android
```
## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

