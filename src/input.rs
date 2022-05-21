use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;

use crate::config::KeyBindConfig;
use std::fmt::Formatter;
use crate::input::Axis::{MouseX, MouseY};


pub struct KeyBindJustPressedEvent(pub String);
pub struct KeyBindPressedEvent(pub String);
pub struct KeyBindJustReleasedEvent(pub String);

pub struct KeyBindAxisEvent(pub String, pub f32);

pub struct KeyBindPlugin(pub KeyBindConfig);

impl KeyBindPlugin {
    pub fn new(config_file: Option<String>) -> Self {
        KeyBindPlugin(KeyBindConfig::new(config_file))
    }
}

impl Plugin for KeyBindPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.0.clone())
            .add_event::<KeyBindJustPressedEvent>()
            .add_event::<KeyBindPressedEvent>()
            .add_event::<KeyBindJustReleasedEvent>()
            .add_event::<KeyBindAxisEvent>()
            .add_system(handle_key_input)
            .add_system(handle_mouse_input)
            .add_system(handle_mouse_motion)
        ;
    }
}


fn handle_key_input(
    keys: Res<Input<KeyCode>>,
    bindings: Res<KeyBindConfig>,
    mut just_pressed: EventWriter<KeyBindJustPressedEvent>,
    mut pressed: EventWriter<KeyBindPressedEvent>,
    mut just_released: EventWriter<KeyBindJustReleasedEvent>,
) {
    keys.get_just_pressed().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(&it) {
            Some((_key, value)) => {
                just_pressed.send(KeyBindJustPressedEvent(value.clone()));
            },
            None => { }
        }
    });

    keys.get_pressed().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(&it) {
            Some((_key, value)) => {
                pressed.send(KeyBindPressedEvent(value.clone()));
            },
            None => { }
        }
    });

    keys.get_just_released().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(&it) {
            Some((_key, value)) => {
                just_released.send(KeyBindJustReleasedEvent(value.clone()));
            },
            None => { }
        }
    })
}


fn handle_mouse_input(
    mouse_buttons: Res<Input<MouseButton>>,
    bindings: Res<KeyBindConfig>,
) {
    mouse_buttons.get_just_pressed().for_each(|it| {});

    mouse_buttons.get_pressed().for_each(|it| {});

    mouse_buttons.get_just_released().for_each(|it| {});
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum Axis {
    MouseX,
    MouseY
}

fn handle_mouse_motion(
    mut mouse_motion: EventReader<MouseMotion>,
    mut axis_event: EventWriter<KeyBindAxisEvent>,
    bindings: Res<KeyBindConfig>,
) {
    mouse_motion.iter().for_each(|it: &MouseMotion| {
        match bindings.axes.get_key_value(&MouseX) {
            Some((_key, value)) => {
                axis_event.send(KeyBindAxisEvent(value.clone(), it.delta.x.clone()))
            },
            None => { }
        }

        match bindings.axes.get_key_value(&MouseY) {
            Some((_key, value)) => {
                axis_event.send(KeyBindAxisEvent(value.clone(), it.delta.y.clone()))
            },
            None => { }
        }
    });
}
