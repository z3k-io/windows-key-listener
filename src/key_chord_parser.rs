use std::collections::HashMap;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Parser for key chord strings like "Ctrl + Shift + A" into Windows virtual key codes.
pub struct KeyChordParser {
    key_map: HashMap<String, i32>,
}

impl KeyChordParser {
    /// Creates a new KeyChordParser with mappings for supported key names.
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

        // Alphanumeric keys
        key_map.insert("a".to_string(), 0x41);
        key_map.insert("b".to_string(), 0x42);
        key_map.insert("c".to_string(), 0x43);
        key_map.insert("d".to_string(), 0x44);
        key_map.insert("e".to_string(), 0x45);
        key_map.insert("f".to_string(), 0x46);
        key_map.insert("g".to_string(), 0x47);
        key_map.insert("h".to_string(), 0x48);
        key_map.insert("i".to_string(), 0x49);
        key_map.insert("j".to_string(), 0x4A);
        key_map.insert("k".to_string(), 0x4B);
        key_map.insert("l".to_string(), 0x4C);
        key_map.insert("m".to_string(), 0x4D);
        key_map.insert("n".to_string(), 0x4E);
        key_map.insert("o".to_string(), 0x4F);
        key_map.insert("p".to_string(), 0x50);
        key_map.insert("q".to_string(), 0x51);
        key_map.insert("r".to_string(), 0x52);
        key_map.insert("s".to_string(), 0x53);
        key_map.insert("t".to_string(), 0x54);
        key_map.insert("u".to_string(), 0x55);
        key_map.insert("v".to_string(), 0x56);
        key_map.insert("w".to_string(), 0x57);
        key_map.insert("x".to_string(), 0x58);
        key_map.insert("y".to_string(), 0x59);
        key_map.insert("z".to_string(), 0x5A);

        key_map.insert("0".to_string(), 0x30);
        key_map.insert("1".to_string(), 0x31);
        key_map.insert("2".to_string(), 0x32);
        key_map.insert("3".to_string(), 0x33);
        key_map.insert("4".to_string(), 0x34);
        key_map.insert("5".to_string(), 0x35);
        key_map.insert("6".to_string(), 0x36);
        key_map.insert("7".to_string(), 0x37);
        key_map.insert("8".to_string(), 0x38);
        key_map.insert("9".to_string(), 0x39);

        // Function keys
        key_map.insert("f1".to_string(), 0x70);
        key_map.insert("f2".to_string(), 0x71);
        key_map.insert("f3".to_string(), 0x72);
        key_map.insert("f4".to_string(), 0x73);
        key_map.insert("f5".to_string(), 0x74);
        key_map.insert("f6".to_string(), 0x75);
        key_map.insert("f7".to_string(), 0x76);
        key_map.insert("f8".to_string(), 0x77);
        key_map.insert("f9".to_string(), 0x78);
        key_map.insert("f10".to_string(), 0x79);
        key_map.insert("f11".to_string(), 0x7A);
        key_map.insert("f12".to_string(), 0x7B);

        // Numpad keys
        key_map.insert("num0".to_string(), 0x60);
        key_map.insert("num1".to_string(), 0x61);
        key_map.insert("num2".to_string(), 0x62);
        key_map.insert("num3".to_string(), 0x63);
        key_map.insert("num4".to_string(), 0x64);
        key_map.insert("num5".to_string(), 0x65);
        key_map.insert("num6".to_string(), 0x66);
        key_map.insert("num7".to_string(), 0x67);
        key_map.insert("num8".to_string(), 0x68);
        key_map.insert("num9".to_string(), 0x69);

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

        KeyChordParser { key_map }
    }

    /// Parses a key chord string into Windows virtual key codes.
    /// Format: key names separated by '+', e.g., "Ctrl + Shift + Z"
    pub fn parse(&self, key_chord: &str) -> Option<Vec<i32>> {
        let mut vk_codes = Vec::new();

        for key in key_chord.split('+').map(|s| s.trim().to_lowercase()) {
            if let Some(&vk_code) = self.key_map.get(&key) {
                vk_codes.push(vk_code);
            } else {
                return None;
            }
        }

        if vk_codes.is_empty() {
            None
        } else {
            Some(vk_codes)
        }
    }
}
