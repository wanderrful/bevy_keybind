use std::collections::HashMap;

use bevy::input::keyboard::KeyCode;
use serde_yaml;

use crate::input::Axis;

const DEFAULT_FILE_PATH: &str = "assets/bindings.yaml";

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct KeyBindConfig {
    pub actions: HashMap<KeyCode, String>,
    pub axes: HashMap<Axis, String>,
}

impl KeyBindConfig {
    pub fn new(input_file: Option<String>) -> Self {
        let input_file_unwrapped = input_file.unwrap_or(DEFAULT_FILE_PATH.parse().unwrap());
        let error_input_file_not_found: String = format!(
            "[bevy_keybind] Input file '{}' was not found! Is the path correct?",
            input_file_unwrapped);
        let error_input_file_formatting: String = format!(
            "[bevy_keybind] Input file '{}' failed YAML validation! Is it formatted properly? \
Maybe a KeyCode is not recognized by Bevy?",
            input_file_unwrapped);

        let bindings: KeyBindConfig = serde_yaml::from_reader(
            std::io::BufReader::new(
                std::fs::File::open(input_file_unwrapped
                ).expect(error_input_file_not_found.as_str())
            )
        ).expect(error_input_file_formatting.as_str());


        bindings
    }
}

impl Default for KeyBindConfig {
    fn default() -> Self {
        KeyBindConfig::new(None)
    }
}
