# Hyogen-UI

Hyogen-UI is a background UI module for the Aurora compositor, designed to render expressive and dynamic animations. It serves as the visual representation of the face animations for Osmos's compositor Aurora.
The name *Hyogen* is inspired by the Japanese word **表現 (Hyōgen)**, which means "expression" or "representation." This reflects the module's purpose of bringing life and emotion to the Osmos system through expressive animations and visuals.

## Features
- Renders textures on the background layer of Aurora.
- Supports dynamic SVG-based face animations.
- Integrates seamlessly with the Aurora compositor.

## Getting Started
This guide will help you set up and run Hyogen-UI.

### Prerequisites
- **Rust**: Install Rust from [rustup.rs](https://rustup.rs/).
- **Aurora Compositor**: Ensure that [Aurora](https://github.com/IshantPundir/aurora) is running as your Wayland compositor.

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/IshantPundir/hyogen-ui.git
   cd hyogen-ui
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run Hyogen-UI (development):
   ```bash
   WAYLAND_DISPLAY=wayland-2 cargo run    
   ```

## Usage
Hyogen-UI runs as a module alongside Aurora. Once started, it renders textures on the background layer of the compositor. In its current state, it displays a static texture. Future updates will include dynamic SVG animations and interactive elements.

### Roadmap
- [x] Render a static texture on the background layer.
- [ ] Implement dynamic SVG-based animations.
- [ ] Add support for real-time interaction.