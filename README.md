# About
Iced Spatial Navigation is an unofficial extension to the Iced UI framework.<br>
It is designed to bring easy remote or gamepad navigation to any TV/console style app.<br>

# Setup
In order to set up Iced Spatial Navigation in your project, simply type:<br>

```bash
cargo add iced_spatial_navigation
```

Or, in your `Cargo.toml` file:

`iced_spatial_navigation = { git = "https://github.com/JaydonXOneGitHub/iced_spatial_navigation", branch = "master" }`

# Use in Iced application
Set it up in a few easy steps.<br>
In `main`, you can keep it as-is:<br>

```rust
fn main() -> Result<(), iced::Error> {
    return iced::application(initialize, update, view)
    .subscription(subscription)
    .scale_factor(Environment::get_scale_factor)
    .run();
}
```

However, here's where it's probably going to get a little weird.<br>
In `main.rs`, or another file, make a custom enum:<br>
```rust
pub enum CustomMessage { /* Fill in fields */
```

Similarly, define a `GridButton` struct that implements `TGridButton`:

```rust
pub struct GridButton {}

impl TGridButton for GridButton {
    // How exactly it's done is up to you.
}
```

Then, use this as the `initialize` function:

```rust
fn initialize() -> Environment<CustomMessage, GridButton> { // Make sure there are accessible
    Environment::new(|| Grid::new().with_tile_size(150.0).with_spacing(10.0))
}
```
