//! A Windows library for listening to global keyboard shortcuts.
//! 
//! Register global keyboard shortcuts as arbitrary chords and execute callbacks when they are triggered.
//!
//! # Logging
//! 
//! This library uses the `log` crate for configurable logging.
//! 
//! # Example
//! ```no_run
//! use windows_key_listener::KeyListener;
//! use std::{sync::Arc, time::Duration};
//!
//! let listener = KeyListener::new();
//! 
//! // Listen for Ctrl+Shift+Z with 200ms debounce
//! match listener.listen(
//!     "Ctrl + Shift + Z",
//!     Duration::from_millis(200),
//!     Arc::new(|| {
//!         println!("Shortcut triggered!");
//!         false // Return true to block the key event
//!     })
//! ) {
//!     Ok(_) => println!("Shortcut registered successfully"),
//!     Err(e) => eprintln!("Failed to register shortcut: {}", e),
//! }
//!
//! // Run the message loop to process key events
//! listener.run_message_loop();
//! ```

mod key_chord_parser;
mod key_listener;

pub use key_listener::{KeyListener, Callback, KeyListenerError};
pub use log;
