# rwlogout

A modern logout/shutdown dialog for Linux desktop environments, built with Rust and GTK4. This application provides a clean interface for system power management operations.

## Features

✨ **Modern UI Framework**: Built with [relm4](https://github.com/Relm4/Relm4) and GTK4
🔧 **Configurable**: KDL-based configuration for easy customization
⌨️ **Keyboard Support**: Configurable keybindings for all actions
🖱️ **Mouse Friendly**: Hover-to-focus and click interactions
🏷️ **Layer Shell**: Uses gtk4-layer-shell for proper overlay display
🎨 **Themeable**: CSS-based styling support

## Supported Operations

- **Lock** - Lock the current session
- **Logout** - Terminate user session
- **Shutdown** - Power off the system
- **Reboot** - Restart the system
- **Suspend** - Suspend to RAM
- **Hibernate** - Suspend to disk

## Installation

### Using Cargo

```bash
# Install dependencies (Debian/Ubuntu)
sudo apt install libgtk-4-dev libglib2.0-dev pkg-config

# Install dependencies (Arch Linux)
sudo pacman -S gtk4 glib2 pkg-config

# Clone and build
git clone https://github.com/dwapp/rwlogout.git
cd rwlogout
cargo build --release
```

### Using Nix

```bash
# Build with Nix
nix build

# Or run directly
nix run
```

### Development Environment

```bash
# Enter development shell (Nix)
nix develop

# Or use traditional method
nix-shell -p glib.dev pkg-config zlib cargo pango gdk-pixbuf gtk4 gtk4-layer-shell
```

## Configuration

The application is configured via the `layout.kdl` file using the [KDL](https://kdl.dev/) format:

```kdl
// Layout configuration for rwlogout
button "lock" {
    action "loginctl lock-session"
    text "Lock"
    keybind "l"
}

button "shutdown" {
    action "systemctl poweroff"
    text "Shutdown"
    keybind "s"
}

// Add more buttons as needed...
```

### Configuration Fields

- **`label`**: Internal identifier for the button
- **`action`**: System command to execute
- **`text`**: Display text shown on the button
- **`keybind`**: Single character keyboard shortcut

### Environment Variables

Commands support environment variable expansion:
- `$USER` - Current username
- Standard shell environment variables

## Usage

### Running the Application

```bash
# Run directly
cargo run

# Or run the built binary
./target/release/rwlogout
```

### Keyboard Shortcuts

- **L** - Lock screen
- **E** - Logout
- **S** - Shutdown
- **R** - Reboot
- **U** - Suspend
- **H** - Hibernate
- **Escape** - Exit application

### Mouse Interaction

- **Click** - Execute the button's action
- **Hover** - Automatically focus the button
- **Background Click** - Exit application

## Architecture

The application uses a modern Rust architecture:

- **relm4**: Reactive GUI framework built on GTK4
- **KDL**: Human-friendly configuration format
- **gtk4-layer-shell**: Wayland layer shell support for overlay display
- **Dynamic UI**: Buttons generated from configuration file

### Key Components

```
src/
├── main.rs      # Main application and UI logic
├── config.rs    # Configuration parsing and command execution
└── style.css    # UI styling
layout.kdl       # Button configuration
```

## Styling

The application supports CSS-based theming via `src/style.css`. You can customize:

- Button appearance and colors
- Layout spacing and sizing
- Hover and focus effects
- Font and typography

## Wayland Support

The application uses gtk4-layer-shell for proper integration with Wayland compositors. It will:

- Display as an overlay on all screens
- Grab keyboard focus when shown
- Handle layer shell protocols correctly

On X11 systems, it falls back to standard fullscreen window behavior.

## Development

### Architecture Overview

The application follows the Model-View-Update (MVU) pattern via relm4:

1. **Model**: Application state and configuration
2. **View**: Declarative UI definition
3. **Update**: Message handling for user interactions

### Key Features

- **Dynamic UI Generation**: Buttons are created based on configuration
- **Message-Driven**: All interactions go through the message system
- **Automatic Cleanup**: Application exits after executing commands

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Similar Projects

- [wlogout](https://github.com/ArtsyMacaw/wlogout) - Original inspiration
- [wleave](https://github.com/AMNatty/wleave) - Alternative Wayland logout menu
- [pwrmenu](https://github.com/kamilernerd/pwrmenu) - Power menu for window managers
- [rlogout](https://github.com/MarcusBoay/rlogout) - Rust-based logout utility

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [relm4](https://github.com/Relm4/Relm4) reactive GUI framework
- Uses [KDL](https://kdl.dev/) for configuration
- Inspired by the original [wlogout](https://github.com/ArtsyMacaw/wlogout) project

---

> This project serves as both a practical utility and a learning exercise for Rust and modern GTK4 development.
