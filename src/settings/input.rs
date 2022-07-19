use std::{collections::HashMap, rc::Rc, cell::RefCell, hash::Hash};
use serde::{Serialize, Deserialize, Deserializer};
use crate::input::{InputId, InputConfiguration, keyboard::Keyboards};
use super::{MAX_PLAYERS};

pub(crate) type InputConfigurationRef = Rc<RefCell<InputConfiguration>>;

#[derive(Debug)]
pub(crate) struct InputSettings {
    pub(crate) selected: [InputConfigurationRef; MAX_PLAYERS],
    pub(crate) configurations: HashMap<InputId, InputConfigurationRef>
}

impl InputSettings {
    pub(crate) fn get_or_create_config(&mut self, id: &InputId, default: InputConfiguration) -> &InputConfigurationRef {
        self.configurations.entry(id.clone()).or_insert_with(|| Rc::new(RefCell::new(default)))
    }
    pub(crate) fn get_default_config(&mut self, player: usize) -> &InputConfigurationRef {
        let default = Keyboards::default_configurations(player);
        self.get_or_create_config(&default.id.clone(), default)
    }
}

impl Hash for InputSettings {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.selected[0].borrow().hash(state);
        self.selected[1].borrow().hash(state);
        
        for (k, v) in &self.configurations {
            k.hash(state);
            v.borrow().hash(state);
        }
    }
}

#[derive(Serialize, Deserialize)]
struct SerializableInputSettings {
    selected: [InputId; MAX_PLAYERS],
    configurations: HashMap<InputId, InputConfiguration>
}

impl SerializableInputSettings {
    fn new(source: &InputSettings) -> Self {
        SerializableInputSettings {
            selected: source.selected.clone().map(|v| v.borrow().id.clone()),
            configurations: source.configurations.iter().map(|(k, v)| (k.clone(), v.borrow().clone())).collect()
        }
    }
}

impl Serialize for InputSettings {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        SerializableInputSettings::new(self).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for InputSettings {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
        SerializableInputSettings::deserialize(deserializer).map(InputSettings::from)
    }
}

impl InputSettings {
    fn from(source: SerializableInputSettings) -> Self {
        let configurations: HashMap<InputId, InputConfigurationRef> = source.configurations.iter().map(|(k, v)| (k.clone(), Rc::new(RefCell::new(v.clone())))).collect();
        let selected = [
            Rc::clone(configurations.get(&source.selected[0]).expect("non-existant configuration selected")),
            Rc::clone(configurations.get(&source.selected[1]).expect("non-existant configuration selected"))
        ];
        Self {
            selected,
            configurations,
        }
    }
}