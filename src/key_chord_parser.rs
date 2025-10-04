use std::collections::HashMap;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub struct KeyChordParser {
    key_map: HashMap<String, i32>,
}

impl KeyChordParser {
    pub fn new() -> Self {
        let mut key_map = HashMap::new();

        // Modifier keys
        key_map.insert("ctrl".to_string(), VK_CONTROL.0 as i32);
        key_map.insert("lctrl".to_string(), VK_LCONTROL.0 as i32);
        key_map.insert("rctrl".to_string(), VK_RCONTROL.0 as i32);

        key_map.insert("shift".to_string(), VK_SHIFT.0 as i32);
        key_map.insert("lshift".to_string(), VK_LSHIFT.0 as i32);
        key_map.insert("rshift".to_string(), VK_RSHIFT.0 as i32);

        key_map.insert("alt".to_string(), VK_MENU.0 as i32);
        key_map.insert("lalt".to_string(), VK_LMENU.0 as i32);
        key_map.insert("ralt".to_string(), VK_RMENU.0 as i32);

        // Alphabet keys
        for (i, key) in ('a'..='z').enumerate() {
            key_map.insert(key.to_string(), 0x41 + i as i32);
        }

        // Number keys
        for (i, key) in ('0'..='9').enumerate() {
            key_map.insert(key.to_string(), 0x30 + i as i32);
        }

        // Function keys
        for i in 1..=12 {
            key_map.insert(format!("f{}", i), 0x6F + i as i32);
        }

        // Numpad keys
        for i in 0..=9 {
            key_map.insert(format!("num{}", i), 0x60 + i as i32);
        }

        // Numpad special keys
        key_map.insert("numlock".to_string(), VK_NUMLOCK.0 as i32);
        key_map.insert("numslash".to_string(), VK_DIVIDE.0 as i32);
        key_map.insert("nummultiply".to_string(), VK_MULTIPLY.0 as i32);
        key_map.insert("numminus".to_string(), VK_SUBTRACT.0 as i32);
        key_map.insert("numplus".to_string(), VK_ADD.0 as i32);
        key_map.insert("numenter".to_string(), VK_RETURN.0 as i32);
        key_map.insert("numdecimal".to_string(), VK_DECIMAL.0 as i32);

        // Special keys
        key_map.insert("back".to_string(), VK_BACK.0 as i32);
        key_map.insert("tab".to_string(), VK_TAB.0 as i32);
        key_map.insert("enter".to_string(), VK_RETURN.0 as i32);
        key_map.insert("space".to_string(), VK_SPACE.0 as i32);
        key_map.insert("capslock".to_string(), VK_CAPITAL.0 as i32);
        key_map.insert("esc".to_string(), VK_ESCAPE.0 as i32);

        // Navigation keys
        key_map.insert("left".to_string(), VK_LEFT.0 as i32);
        key_map.insert("right".to_string(), VK_RIGHT.0 as i32);
        key_map.insert("up".to_string(), VK_UP.0 as i32);
        key_map.insert("down".to_string(), VK_DOWN.0 as i32);
        key_map.insert("home".to_string(), VK_HOME.0 as i32);
        key_map.insert("end".to_string(), VK_END.0 as i32);
        key_map.insert("pageup".to_string(), VK_PRIOR.0 as i32);
        key_map.insert("pagedown".to_string(), VK_NEXT.0 as i32);
        key_map.insert("insert".to_string(), VK_INSERT.0 as i32);
        key_map.insert("delete".to_string(), VK_DELETE.0 as i32);

        // System keys
        key_map.insert("printscreen".to_string(), VK_SNAPSHOT.0 as i32);
        key_map.insert("scrolllock".to_string(), VK_SCROLL.0 as i32);
        key_map.insert("pause".to_string(), VK_PAUSE.0 as i32);
        key_map.insert("break".to_string(), VK_CANCEL.0 as i32);
        key_map.insert("menu".to_string(), VK_MENU.0 as i32);
        key_map.insert("lmenu".to_string(), VK_LMENU.0 as i32);
        key_map.insert("rmenu".to_string(), VK_RMENU.0 as i32);
        key_map.insert("lwin".to_string(), VK_LWIN.0 as i32);
        key_map.insert("rwin".to_string(), VK_RWIN.0 as i32);
        key_map.insert("apps".to_string(), VK_APPS.0 as i32);
        key_map.insert("sleep".to_string(), VK_SLEEP.0 as i32);
        key_map.insert("zoom".to_string(), VK_ZOOM.0 as i32);

        // Media keys
        key_map.insert("volumeup".to_string(), VK_VOLUME_UP.0 as i32);
        key_map.insert("volumedown".to_string(), VK_VOLUME_DOWN.0 as i32);
        key_map.insert("volumemute".to_string(), VK_VOLUME_MUTE.0 as i32);
        key_map.insert("stop".to_string(), VK_MEDIA_STOP.0 as i32);
        key_map.insert("playpause".to_string(), VK_MEDIA_PLAY_PAUSE.0 as i32);
        key_map.insert("prev".to_string(), VK_MEDIA_PREV_TRACK.0 as i32);
        key_map.insert("next".to_string(), VK_MEDIA_NEXT_TRACK.0 as i32);

        Self { key_map }
    }

    /// Parses a key chord string into Windows virtual key codes.
    /// Format: key names separated by '+', e.g., "Ctrl + Shift + Z"
    pub fn parse(&self, key_chord: &str) -> Option<Vec<i32>> {
        let vk_codes: Vec<i32> = key_chord
            .split('+')
            .map(|s| s.trim().to_lowercase())
            .filter_map(|key| self.key_map.get(&key).copied())
            .collect();

        if vk_codes.is_empty() { None } else { Some(vk_codes) }
    }
}
