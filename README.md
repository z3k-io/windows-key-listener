# Windows Key Listener

A Rust library for global keyboard event listening and hotkey management on Windows.

## Usage

```rust
use windows_key_listener::KeyListener;
use std::{sync::Arc, time::Duration};

fn main() {
    let listener = KeyListener::new();

    // Register shortcuts
    key_listener.listen(
        "Ctrl + Shift + A", 
        Duration::from_millis(200),
        Arc::new(|| {
            on_key_pressed();
            false   // Return true to block the event
        })
    );

    run_your_app();
    
    key_listener.unlisten();
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
windows-key-listener = "x.y.z"
```

## License

[MIT License](LICENSE)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
