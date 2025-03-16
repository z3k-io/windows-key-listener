#![allow(unused)]

use lazy_static::lazy_static;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, HINSTANCE, HANDLE};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::INFINITE;
use windows::Win32::UI::Input::KeyboardAndMouse::GetKeyState;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    PeekMessageW, PM_REMOVE, TranslateMessage, DispatchMessageW, 
    MsgWaitForMultipleObjects, MWMO_INPUTAVAILABLE, QS_ALLINPUT
};
use windows::core::Error as WindowsError;

use crate::key_chord_parser::KeyChordParser;

/// Return true to block the key event, false to allow it to propagate.
pub type Callback = Arc<dyn Fn() -> bool + Send + Sync + 'static>;

/// Errors that can occur when using KeyListener.
#[derive(Debug)]
pub enum KeyListenerError {
    /// Invalid key chord format or unrecognized key name
    InvalidKeyChord(String),
    /// Error from Windows API
    WindowsError(WindowsError),
    /// Other internal error
    InternalError(String),
}

impl fmt::Display for KeyListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyListenerError::InvalidKeyChord(chord) => write!(f, "Invalid key chord: {}", chord),
            KeyListenerError::WindowsError(e) => write!(f, "Windows API error: {}", e),
            KeyListenerError::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for KeyListenerError {}

struct HookData {
    callback: Callback,
    vk_codes: Vec<i32>,
    debounce_interval: Duration,
    last_trigger: Arc<RwLock<Instant>>,
    hook_handle: Arc<RwLock<Option<isize>>>,
}

lazy_static! {
    static ref HOOKS: Arc<RwLock<Vec<HookData>>> = Arc::new(RwLock::new(Vec::new()));
    static ref GLOBAL_KEY_STATES: Arc<RwLock<HashMap<i32, bool>>> =
        Arc::new(RwLock::new(HashMap::new()));
    static ref MESSAGE_LOOP_STARTED: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
}

fn handle_chord_pressed(hook: &HookData) -> bool {
    GLOBAL_KEY_STATES.write().unwrap().clear();

    // Check if we should trigger based on the debounce_interval
    let should_trigger = {
        let mut last_trigger = hook.last_trigger.write().unwrap();
        let now = Instant::now();
        let should_fire = now.duration_since(*last_trigger) >= hook.debounce_interval;
        if should_fire {
            *last_trigger = now;
        }
        should_fire
    };

    if should_trigger {
        debug!("Triggering callback");
        let callback = Arc::clone(&hook.callback);
        let callback_result = callback();
        if callback_result {
            debug!("Stopping propagation");
            return true;
        } else {
            debug!("Allowing propagation");
            return false;
        }
    } else {
        debug!("Skipping callback due to debounce_interval");
    }

    return false;
}

fn normalize_keys(keys: Vec<i32>) -> Vec<i32> {
    keys.iter()
        .map(|key| match key {
            160 | 161 => 16, // Left/Right Shift -> Shift (VK_SHIFT)
            162 | 163 => 17, // Left/Right Ctrl -> Ctrl (VK_CONTROL)
            164 | 165 => 18, // Left/Right Alt -> Alt (VK_MENU)
            91 | 92 => 91,   // Left/Right Win -> Win (VK_LWIN)
            _ => *key,
        })
        .collect()
}

unsafe fn get_pressed_keys() -> Vec<i32> {
    let global_key_states = GLOBAL_KEY_STATES.read().unwrap();
    global_key_states.keys().cloned().collect()
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code != HC_ACTION as i32 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
    }

    let kb_struct = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let virtual_key_code = kb_struct.vkCode as i32;

    let is_key_down = w_param.0 as usize == WM_KEYDOWN as usize;
    let is_key_up = w_param.0 as usize == WM_KEYUP as usize;

    if is_key_down {
        trace!("Key down: {}", virtual_key_code);
        GLOBAL_KEY_STATES
            .write()
            .unwrap()
            .insert(virtual_key_code, true);
    }

    if is_key_up {
        trace!("Key up: {}", virtual_key_code);
        GLOBAL_KEY_STATES.write().unwrap().remove(&virtual_key_code);
    }

    let pressed_keys = unsafe { get_pressed_keys() };
    let normalized_pressed_keys = normalize_keys(pressed_keys);
    trace!("Pressed keys: {:?}", normalized_pressed_keys);

    // if pressed_keys matches any of the chords, trigger the callback
    for hook in HOOKS.read().unwrap().iter() {
        if hook
            .vk_codes
            .iter()
            .all(|&chord_key| normalized_pressed_keys.contains(&chord_key))
        {
            debug!("Chord match detected: {:?}", hook.vk_codes);
            let should_block = handle_chord_pressed(&hook);
            if should_block {
                return LRESULT(1);
            }
        }
    }

    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

/// A listener for global keyboard shortcuts on Windows.
pub struct KeyListener {
    parser: KeyChordParser,
}

impl KeyListener {
    /// Creates a new KeyListener.
    pub fn new() -> Self {
        KeyListener {
            parser: KeyChordParser::new(),
        }
    }

    /// Registers a callback for a keyboard shortcut.
    /// 
    /// # Arguments
    /// 
    /// * `key_chord` - A string representing the key combination, e.g., "Ctrl + Shift + Z"
    /// * `debounce_interval` - Minimum time between callback triggers
    /// * `callback` - A function to call when the key chord is pressed
    /// 
    /// # Returns
    /// 
    /// `Result<(), KeyListenerError>` - Success or error information
    pub fn listen(&self, key_chord: &str, debounce_interval: Duration, callback: Callback) -> Result<(), KeyListenerError> {
        info!("Starting to listen for key chord: {}", key_chord);
        let vk_codes = self.parser.parse(key_chord)
            .ok_or_else(|| KeyListenerError::InvalidKeyChord(key_chord.to_string()))?;
            
        debug!("Parsed key chord into VK codes: {:?}", vk_codes);
        unsafe {
            let h_instance = match GetModuleHandleW(None) {
                Ok(module) => {
                    debug!("Got module handle successfully");
                    Some(HINSTANCE(module.0))
                },
                Err(e) => {
                    warn!("Failed to get module handle: {:?}", e);
                    return Err(KeyListenerError::WindowsError(e));
                }
            };
            
            debug!("Setting up keyboard hook");
            let hook_result = SetWindowsHookExW(
                WH_KEYBOARD_LL,
                Some(keyboard_hook),
                h_instance,
                0,
            ).map_err(|e| {
                warn!("Failed to set keyboard hook: {:?}", e);
                KeyListenerError::WindowsError(e)
            })?;

            let hook = hook_result;
            info!("Hook installed successfully");

            let key_states = Arc::new(RwLock::new(HashMap::new()));
            for &code in &vk_codes {
                key_states.write().unwrap().insert(code, false);
            }

            let mut hooks = HOOKS.write().unwrap();
            hooks.push(HookData {
                callback,
                vk_codes,
                debounce_interval,
                last_trigger: Arc::new(RwLock::new(Instant::now())),
                hook_handle: Arc::new(RwLock::new(Some(hook.0 as isize))),
            });

            // Only start the message loop thread once
            let mut message_loop_started = MESSAGE_LOOP_STARTED.write().unwrap();
            if !*message_loop_started {
                info!("Starting message loop thread");
                *message_loop_started = true;
                
                // Start a message loop in a separate thread
                thread::spawn(move || {
                    debug!("Message loop thread started");
                    
                    let mut msg = MSG::default();
                    loop {
                        unsafe {
                            // Wait for messages instead of polling
                            MsgWaitForMultipleObjects(
                                None, // No handles to wait on
                                false, // Don't wait for all
                                INFINITE, // Wait indefinitely
                                QS_ALLINPUT, // Wait for any input
                            );
                            
                            // Process all available messages
                            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                                trace!("Message received: {:?}", msg.message);
                                TranslateMessage(&msg);
                                DispatchMessageW(&msg);
                            }
                        }
                    }
                });
            }
        }

        Ok(())
    }

    /// Unregisters all keyboard shortcuts.
    pub fn unlisten(&self) {
        info!("Unlistening all hooks");
        let mut hooks = HOOKS.write().unwrap();
        hooks.iter_mut().for_each(|hook| {
            if let Some(hhk) = hook.hook_handle.write().unwrap().take() {
                unsafe { 
                    debug!("Unhooking a keyboard hook");
                    match UnhookWindowsHookEx(HHOOK(hhk as *mut _)) {
                        Ok(_) => debug!("Hook successfully unhooked"),
                        Err(e) => warn!("Failed to unhook: {:?}", e)
                    }
                };
            }
        });
        
        hooks.clear();
        debug!("All hooks cleared");
    }

    /// Runs a message loop in the current thread to process keyboard events.
    /// This method blocks the current thread indefinitely.
    pub fn run_message_loop(&self) {
        info!("Starting message loop in main thread");
        
        let mut msg = MSG::default();
        loop {
            unsafe {
                // Wait for messages instead of polling with a more efficient approach
                let wait_result = MsgWaitForMultipleObjects(
                    None, // No handles to wait on
                    false, // Don't wait for all (not relevant with empty handle array)
                    INFINITE, // Wait indefinitely
                    QS_ALLINPUT, // Wait for any input
                );
                
                // Process all available messages
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
}
