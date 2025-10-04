use std::{sync::Arc, time::Duration};
use windows_key_listener::KeyListener;
use log::LevelFilter;

fn main() {
    env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .init();

    let key_listener = KeyListener::new();
    let debounce_interval = Duration::from_millis(200);

    key_listener.listen(
        "Ctrl + Shift + Q",
        debounce_interval,
        Arc::new(|| {
            println!("Ctrl + Q pressed!");
            false // Propagate the event
        }),
    ).expect("Failed to register Ctrl+Q shortcut");

    key_listener.listen(
        "VolumeUp",
        debounce_interval,
        Arc::new(|| {
            println!("Volume Up pressed!");
            true // Block the event
        }),
    ).expect("Failed to register VolumeUp shortcut");

    key_listener.listen(
        "VolumeDown",
        debounce_interval,
        Arc::new(|| {
            println!("Volume Down pressed!");
            false // Block the event
        }),
    ).expect("Failed to register VolumeUp shortcut");

    log::info!("Listening for key combinations...");
    log::info!("Press Ctrl+Q to test or Ctrl+C to exit");

    // Run the message loop in the main thread
    key_listener.run_message_loop();
}
