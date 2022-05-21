use bevy_input::mouse::{MouseMotion, MouseButton};
use bevy_app::{App, Plugin};
use bevy_ecs::system::Res;
use bevy_input::Input;
use bevy_input::keyboard::KeyCode;
use bevy_ecs::event::{EventWriter, EventReader};

use crate::config::KeyBindConfig;
use crate::input::KeyBindAxis::{MouseX, MouseY};


pub struct KeyBindJustPressedEvent(pub String);
pub struct KeyBindPressedEvent(pub String);
pub struct KeyBindJustReleasedEvent(pub String);

pub struct KeyBindAxisEvent(pub String, pub f32);


#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum KeyBindAxis {
    MouseX,
    MouseY
    // TODO | Add support for other axes, like controller Joysticks, driving wheels, and HOTAS.
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Hash, Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
pub enum KeyBindMouseButton {
    Mouse1,
    Mouse2,
    Mouse3,
    Mouse4,
    Mouse5,
    Mouse6,
    Mouse7,
    Mouse8, // Arbitarily stopping here at 8 "mouse buttons"
}


/// The primary plugin that developers should use for their projects.
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


/// Map Bevy's concept of KeyCode to our concept of KeyBind actions
fn handle_key_input(
    keys: Res<Input<KeyCode>>,
    bindings: Res<KeyBindConfig>,
    mut just_pressed: EventWriter<KeyBindJustPressedEvent>,
    mut pressed: EventWriter<KeyBindPressedEvent>,
    mut just_released: EventWriter<KeyBindJustReleasedEvent>,
) {
    keys.get_just_pressed().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(_to_str(&it).as_str()) {
            Some((_key, value)) => {
                just_pressed.send(KeyBindJustPressedEvent(value.clone()));
            },
            None => { }
        }
    });

    keys.get_pressed().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(_to_str(&it).as_str()) {
            Some((_key, value)) => {
                pressed.send(KeyBindPressedEvent(value.clone()));
            },
            None => { }
        }
    });

    keys.get_just_released().for_each(|&it: &KeyCode| {
        match bindings.actions.get_key_value(_to_str(&it).as_str()) {
            Some((_key, value)) => {
                just_released.send(KeyBindJustReleasedEvent(value.clone()));
            },
            None => { }
        }
    })
}


/// Map the Bevy Engine's concept of mouse inputs to our concept of KeyBind actions.
fn handle_mouse_input(
    mouse_buttons: Res<Input<MouseButton>>,
    bindings: Res<KeyBindConfig>,
    mut just_pressed: EventWriter<KeyBindJustPressedEvent>,
    mut pressed: EventWriter<KeyBindPressedEvent>,
    mut just_released: EventWriter<KeyBindJustReleasedEvent>,
) {
    mouse_buttons.get_just_pressed().for_each(|it: &MouseButton| {
        match bindings.actions.get_key_value(
            _to_str(&KeyBindMouseButton::from(*it)).as_str()
        ) {
            Some((_key, value)) => {
                just_pressed.send(KeyBindJustPressedEvent(value.clone()));
            },
            _ => {}
        }
    });

    mouse_buttons.get_pressed().for_each(|it: &MouseButton| {
        match bindings.actions.get_key_value(
            _to_str(&KeyBindMouseButton::from(*it)).as_str()
        ) {
            Some((_key, value)) => {
                pressed.send(KeyBindPressedEvent(value.clone()));
            },
            _ => {}
        }
    });

    mouse_buttons.get_just_released().for_each(|it: &MouseButton| {
        match bindings.actions.get_key_value(
            _to_str(&KeyBindMouseButton::from(*it)).as_str()
        ) {
            Some((_key, value)) => {
                just_released.send(KeyBindJustReleasedEvent(value.clone()));
            },
            _ => {}
        }
    });
}


/// Map Bevy Engine's concept of MouseMotion to our concept of KeyBindAxis.
fn handle_mouse_motion(
    mut mouse_motion: EventReader<MouseMotion>,
    mut axis_event: EventWriter<KeyBindAxisEvent>,
    bindings: Res<KeyBindConfig>,
) {
    mouse_motion.iter().for_each(|it: &MouseMotion| {
        match bindings.axes.get_key_value(_to_str(&MouseX).as_str()) {
            Some((_key, value)) => {
                axis_event.send(KeyBindAxisEvent(value.clone(), it.delta.x.clone()))
            },
            None => { }
        }

        match bindings.axes.get_key_value(_to_str(&MouseY).as_str()) {
            Some((_key, value)) => {
                axis_event.send(KeyBindAxisEvent(value.clone(), it.delta.y.clone()))
            },
            None => { }
        }
    });
}


/// Serialize the enum name so that we can map it.
fn _to_str<T>(input: &T) -> String
        where T: std::fmt::Debug {
    format!("{:?}", input)
}


/// Map the Bevy Engine's concept of a MouseButton to our concept of a KeyBindMouseButton.
/// NOTE | Bevy's naming conventions for mouse buttons unfortunately collide with the Arrow keys,
///     so, it's not a good user experience for configuring their controls.
impl From<MouseButton> for KeyBindMouseButton {
    fn from(input: MouseButton) -> Self {
        use crate::input::KeyBindMouseButton::*;

        match input {
            MouseButton::Left => Mouse1,
            MouseButton::Right => Mouse2,
            MouseButton::Middle => Mouse3,
            MouseButton::Other(x) => match x {
                0 => Mouse4,
                1 => Mouse5,
                2 => Mouse6,
                3 => Mouse7,
                _ => Mouse8,
            },
        }
    }
}
