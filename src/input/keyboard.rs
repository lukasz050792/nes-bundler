use super::{JoypadInput, JoypadMapping, KeyCode, KeyEvent};
use std::collections::HashSet;

pub type JoypadKeyboardMapping = JoypadMapping<KeyCode>;

pub struct Keyboards {
    pub pressed_keys: HashSet<KeyCode>,
}

impl Keyboards {
    pub fn new() -> Self {
        Keyboards {
            pressed_keys: HashSet::new(),
        }
    }
    pub fn advance(&mut self, key_event: &KeyEvent) {
        match key_event {
            KeyEvent::Pressed(key, _) => self.pressed_keys.insert(*key),
            KeyEvent::Released(key, _) => self.pressed_keys.remove(key),
        };
    }

    pub fn get_joypad(&mut self, mapping: &JoypadKeyboardMapping) -> JoypadInput {
        mapping.calculate_state(&self.pressed_keys)
    }
}
