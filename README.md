# About
Iced Spatial Navigation is an unofficial extension to the Iced UI framework.<br>
It is designed to bring easy remote or gamepad navigation to any TV/console style app.<br><br>

# Features
It is also designed to scale well with any display, completely lossless.<br><br>

It should be able to handle any vertical button grid you can throw at it, with scrolling and all.<br><br>

Fair warning, though: it cannot handle typical horizontal scrolling without slight code modification.<br><br>

# Why did I make this?
I was wanting to make a Linux smart TV distro, but no matter which one I went to,<br>
at least for Plasma 6, Plasma Bigscreen wasn't available.<br><br>

And Kodi smart TV projects were already available (such as [LibreELEC](https://github.com/LibreELEC/LibreELEC.tv)).<br><br>

SteamOS can do something similar, but it's more clunky and made for, you know, gaming.<br><br>

So, I figured, why not try something like this?<br><br>

I'd learnt a fair bit in the process anyway, so even if this amounts to nothing on my end,<br>
it didn't really go to waste.

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
    iced::application(initialize, update, view)
    .subscription(subscription)
    .scale_factor(Environment::get_scale_factor)
    .run()
}
```

However, here's where it's probably going to get a little weird.<br>
In `main.rs`, or another file, make a custom enum:<br>
```rust
pub enum CustomMessage { /* Fill in fields */
```

Make sure to implement `Clone` for it, too, though.<br>

Similarly, define a `GridButton` struct that implements `TGridButton`:<br>

```rust
pub struct GridButton {}

impl TGridButton for GridButton {
    // How exactly it's done is up to you.
}
```

Then, use this as the `initialize` function:<br>

```rust
fn initialize() -> Environment<CustomMessage, GridButton> { // Make sure these custom types are accessible from here
    Environment::new(|| Grid::new().with_tile_size(150.0).with_spacing(10.0)) // Do more with this if you want to preemptively change the Grid object
}
```

Or this:<br>

```rust
fn initialize() -> (Environment<CustomMessage, GridButton>, Task<Message<CustomMessage>>) { // Make sure these custom types are accessible from here
    let env = Environment::new(|| Grid::new().with_tile_size(150.0).with_spacing(10.0).with_padding(10.0)); // Do more with this if you want to preemptively change the Grid object
    let task = Task::none();
    (env, task)
}
```

`update` and `view` should look something like this:<br>

```rust
fn update(_env: &mut Environment<CustomMessage, GridButton>, msg: Message<CustomMessage>) -> Task<Message<CustomMessage>> {
    match msg {
        Message::Event(ev) => {
            Task::none() // Replace this with your own event function call
        },
        Message::ButtonPressed(_pos) => {
            Task::none() // Replace this with your own function call if needed
        },
        Message::ItemSelected(_id) => {
            Task::none() // Replace this with your own function call if needed
        },
        Message::Navigate(dir) => {
            _env.get_grid_mut().move_on_grid(dir)
        }
        _ => Task::none()
    }
}

fn view(_env: &Environment) -> Element<'_, Message<CustomMessage>> {
    row![
        _env.get_grid().to_element()
    ].into()
}
```

As for `subscription`:<br>

```rust
fn subscription(_env: &Environment<CustomMessage, GridButton>) -> Subscription<Message<CustomMessage>> {
    iced::event::listen().map(Message::Event) // Maps to standard Iced events
}
```

And I do believe that's everything needed done to set it up.

# Notice

If you find problems with the library, fork it.<br><br>

I trust that you'll be able to do well and fix the issues. 😊
