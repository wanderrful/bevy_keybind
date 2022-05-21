use bevy::prelude::*;

use bevy_keybind::prelude::*;

/// Initialize a minimal Bevy app with the KeyBindPlugin isolated.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)

        // Just add the plugin! It will automatically read your config YAML file.
        //
        // By default, it assumes `assets/bindings.yaml` is where the config lives.
        //  You can override it by providing a different path manually, instead of 'None'.
        .add_plugin(KeyBindPlugin(KeyBindConfig::default()))

        // Demonstrate that the plugin works
        .add_system(handle_keybind_actions)

        .run();
}

/// Demonstrate that the KeyBindPlugin works as intended.
fn handle_keybind_actions(
    mut just_pressed: EventReader<KeyBindJustPressedEvent>,
    mut pressed: EventReader<KeyBindPressedEvent>,
    mut just_released: EventReader<KeyBindJustReleasedEvent>,
    mut axis_event: EventReader<KeyBindAxisEvent>,
) {
    just_pressed.iter().for_each(|it: &KeyBindJustPressedEvent| {
        // From here, you can invoke your specific game logic based on the action name!
        info!("action event '{:?}' was _just_ pressed!", it.0);
    });

    pressed.iter().for_each(|it: &KeyBindPressedEvent| {
        // From here, you can invoke your specific game logic based on the action name!
        info!("action event '{:?}' continues to be pressed!", it.0);
    });

    just_released.iter().for_each(|it: &KeyBindJustReleasedEvent| {
        // From here, you can invoke your specific game logic based on the action name!
        info!("action event '{:?}' was _just_ released!", it.0);
    });

    axis_event.iter().for_each(|it: &KeyBindAxisEvent| {
        // From here, you can invoke your specific game logic based on the action name!
        info!("axis event '{:?}' received with value '{:?}'", it.0, it.1);
    });
}
