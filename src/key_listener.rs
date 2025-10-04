#![allow(unused)]

use lazy_static::lazy_static;
use log::{debug, info, trace, warn};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::{Duration, Instant};
use windows::Win32::Foundation::{LPARAM, LRESULT, WPARAM, HINSTANCE};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::INFINITE;
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, SetWindowsHookExW, UnhookWindowsHookEx, HC_ACTION, HHOOK,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_KEYUP,
    PeekMessageW, PM_REMOVE, TranslateMessage, DispatchMessageW, 
    MsgWaitForMultipleObjects, QS_ALLINPUT
};
use windows::core::Error as WindowsError;

use crate::key_chord_parser::KeyChordParser;

pub type Callback = Arc<dyn Fn() -> bool + Send + Sync + 'static>;

#[derive(Debug)]
pub enum KeyListenerError {
    InvalidKeyChord(String),
    WindowsError(WindowsError),
    InternalError(String),
}

impl fmt::Display for KeyListenerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidKeyChord(chord) => write!(f, "Invalid key chord: {}", chord),
            Self::WindowsError(e) => write!(f, "Windows API error: {}", e),
            Self::InternalError(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}

impl std::error::Error for KeyListenerError {}

struct HookData {
    callback: Callback,
    vk_codes: Vec<i32>,
    debounce_interval: Duration,
    last_trigger: Arc<RwLock<Instant>>,
}

struct GlobalState {
    hooks: Vec<HookData>,
    key_states: HashMap<i32, bool>,
    hook_handle: Option<isize>,
    message_loop_started: bool,
}

impl GlobalState {
    fn new() -> Self {
        Self {
            hooks: Vec::new(),
            key_states: HashMap::new(),
            hook_handle: None,
            message_loop_started: false,
        }
    }
}

lazy_static! {
    static ref GLOBAL_STATE: Arc<RwLock<GlobalState>> = Arc::new(RwLock::new(GlobalState::new()));
}

const VK_SHIFT: i32 = 16;
const VK_CONTROL: i32 = 17;
const VK_MENU: i32 = 18;
const VK_LWIN: i32 = 91;

fn normalize_key(key: i32) -> i32 {
    match key {
        160 | 161 => VK_SHIFT,
        162 | 163 => VK_CONTROL,
        164 | 165 => VK_MENU,
        91 | 92 => VK_LWIN,
        _ => key,
    }
}

fn get_pressed_keys() -> Vec<i32> {
    GLOBAL_STATE.read().unwrap().key_states.keys().cloned().collect()
}

fn handle_chord_pressed(hook: &HookData) -> bool {
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
        let should_block = (hook.callback)();
        if should_block {
            debug!("Stopping propagation");
        } else {
            debug!("Allowing propagation");
        }
        should_block
    } else {
        debug!("Skipping callback due to debounce_interval");
        true
    }
}

unsafe extern "system" fn keyboard_hook(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code != HC_ACTION as i32 {
        return unsafe { CallNextHookEx(None, n_code, w_param, l_param) };
    }

    let kb_struct = unsafe { &*(l_param.0 as *const KBDLLHOOKSTRUCT) };
    let virtual_key_code = kb_struct.vkCode as i32;
    let is_key_down = w_param.0 as usize == WM_KEYDOWN as usize;
    let is_key_up = w_param.0 as usize == WM_KEYUP as usize;

    {
        let mut state = GLOBAL_STATE.write().unwrap();
        if is_key_down {
            trace!("Key down: {}", virtual_key_code);
            state.key_states.insert(virtual_key_code, true);
        } else if is_key_up {
            trace!("Key up: {}", virtual_key_code);
            state.key_states.remove(&virtual_key_code);
        }
    }

    let pressed_keys = get_pressed_keys();
    let normalized_keys: Vec<i32> = pressed_keys.into_iter().map(normalize_key).collect();
    trace!("Pressed keys: {:?}", normalized_keys);

    let hooks = GLOBAL_STATE.read().unwrap();
    for hook in hooks.hooks.iter() {
        if hook.vk_codes.iter().all(|&chord_key| normalized_keys.contains(&chord_key)) {
            debug!("Chord match detected: {:?}", hook.vk_codes);
            if handle_chord_pressed(hook) {
                debug!("Blocking key event due to chord match");
                return LRESULT(1);
            }
        }
    }

    unsafe { CallNextHookEx(None, n_code, w_param, l_param) }
}

fn setup_global_hook() -> Result<(), KeyListenerError> {
    let mut state = GLOBAL_STATE.write().unwrap();
    if state.hook_handle.is_some() {
        return Ok(());
    }

    unsafe {
        let h_instance = GetModuleHandleW(None)
            .map(|module| Some(HINSTANCE(module.0)))
            .map_err(|e| {
                warn!("Failed to get module handle: {:?}", e);
                KeyListenerError::WindowsError(e)
            })?;

        debug!("Setting up global keyboard hook");
        let hook_result = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), h_instance, 0)
            .map_err(|e| {
                warn!("Failed to set keyboard hook: {:?}", e);
                KeyListenerError::WindowsError(e)
            })?;

        state.hook_handle = Some(hook_result.0 as isize);
        info!("Global hook installed successfully");
    }

    Ok(())
}

fn start_message_loop() {
    let mut state = GLOBAL_STATE.write().unwrap();
    if state.message_loop_started {
        return;
    }

    info!("Starting message loop thread");
    state.message_loop_started = true;

    thread::spawn(|| {
        debug!("Message loop thread started");
        let mut msg = MSG::default();
        loop {
            unsafe {
                MsgWaitForMultipleObjects(None, false, INFINITE, QS_ALLINPUT);
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    trace!("Message received: {:?}", msg.message);
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    });
}

pub struct KeyListener {
    parser: KeyChordParser,
}

impl KeyListener {
    pub fn new() -> Self {
        Self {
            parser: KeyChordParser::new(),
        }
    }

    pub fn listen(&self, key_chord: &str, debounce_interval: Duration, callback: Callback) -> Result<(), KeyListenerError> {
        info!("Starting to listen for key chord: {}", key_chord);
        let vk_codes = self.parser.parse(key_chord)
            .ok_or_else(|| KeyListenerError::InvalidKeyChord(key_chord.to_string()))?;

        debug!("Parsed key chord into VK codes: {:?}", vk_codes);

        {
            let mut state = GLOBAL_STATE.write().unwrap();
            state.hooks.push(HookData {
                callback,
                vk_codes,
                debounce_interval,
                last_trigger: Arc::new(RwLock::new(Instant::now())),
            });
        }

        setup_global_hook()?;
        start_message_loop();

        Ok(())
    }

    pub fn unlisten(&self) {
        info!("Unlistening all hooks");

        let mut state = GLOBAL_STATE.write().unwrap();
        state.hooks.clear();
        debug!("All hook data cleared");

        if let Some(hhk) = state.hook_handle.take() {
            unsafe {
                debug!("Unhooking global keyboard hook");
                match UnhookWindowsHookEx(HHOOK(hhk as *mut _)) {
                    Ok(_) => debug!("Global hook successfully unhooked"),
                    Err(e) => warn!("Failed to unhook global hook: {:?}", e)
                }
            };
        }
    }

    pub fn run_message_loop(&self) {
        info!("Starting message loop in main thread");

        let mut msg = MSG::default();
        loop {
            unsafe {
                MsgWaitForMultipleObjects(None, false, INFINITE, QS_ALLINPUT);
                while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).as_bool() {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                }
            }
        }
    }
}