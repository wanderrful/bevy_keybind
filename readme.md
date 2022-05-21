# Bevy_Keybind

This is a helper plugin for the Bevy game engine. It adds Unreal Engine-like keybind functionality, on top of the
existing engine-level Input features.


## Use Case

The Bevy Engine separates similar concepts in ways that we cannot generalize in the Rust code: for example, a `KeyCode`
and a `MouseButton` are completely separate concepts, but in our projects we need to be able to anonymously assign some
kind of game action to either one. So, we need to write one handler for the KeyCode type and then effectively copy-paste
that same logic so that it can also support the MouseButton type.

A trait would normally work here, but we can't use that because the Bevy Engine does not support using Traits
as Components, and so things like Events and Queries are not compatible there.

So, the pain point we solve with this project is that as developers we should be able to separate the input logic from
the game logic.

Our solution for this is to leverage the Event feature of the Bevy Engine so that all the different types of device
inputs will be broadcasted under a unified input concept, and so instead of reacting to Input events directly we will
react to "KeyBind" events that we can define and configure in a YAML file.


## How to Use

Simply create a YAML file with the following specification, add the `KeyBindPlugin` to your project, and it will
automatically emit action events whenever the given key is pressed according to your configuration!

Example YAML file:
```yaml
actions:
  W: MoveForward
  A: MoveLeft
  S: MoveBack
  D: MoveRight
  C: Crouch
  Mouse1: Fire
  Mouse2: Zoom
  Mouse3: PushToTalk

axes:
  MouseX: LookRight
  MouseY: LookUp
```

The `actions:` type of event emits three types of events:
- `KeyBindActionJustPressedEvent`
- `KeyBindActionPressedEvent`
- `KeyBindActionJustReleasedEvent`

The `axes:` type of event emits one type of event:
- `KeyBindAxisEvent` (contains a payload of `f32`, which represents the magnitude of the axis motion).


So, using the above key binding example, whenever you press and release the W button on your keyboard you can
expect each of the above `KeyBindAction...` events to be broadcasted with a payload of `MoveForward` as a `String` type.


## Benefits

The benefit we get from this design is that you can separate the game logic and the user inputs, and allow users to
configure it themselves via the YAML file!

More specifically, you can use these `EventReader<KeyBind...Event>` in your ECS Systems, and react to the events
accordingly. This way, you don't have to worry about hard coding specific KeyCodes and whatnot in your game logic.
Instead, you can just set your game up to react to the Strings you specify in your YAML file! You can even take it
further, by making each of those Strings (e.g. `MoveForward`, `LookRight`, etc) be an enum and then map the enums to
actual game actions.
