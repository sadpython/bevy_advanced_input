# Bevy advanced inputs
This plugin provide functionality for bindig axises and keysets in bevy.

### Create input types and bindings
```rust
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum InputType {
    Editor,
    MainMenu,
    Game,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Bindings {
    Hotkeys(HotkeysInput),
    Movement(MovementInput),
    Camera(CameraInput),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum MovementInput {
    Forward,
    Right,
    Up,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum CameraInput {
    Yaw,
    Pitch,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum HotkeysInput {
    Test,
}
```

### Binding input
```rust
let mut set = UserInputSet::new()

set.begin_key(Bindings::Hotkeys(HotkeysInput::Test))
    .add(&[
        InputAxisType::KeyboardButton(KeyCode::Q),
        InputAxisType::KeyboardButton(KeyCode::W),
    ])
    .enable_repeat_all_for_reactivation();

set.begin_axis(Bindings::Movement(MovementInput::Forward))
    .add(InputAxisType::KeyboardButton(KeyCode::W), Some(1.0))
    .add(InputAxisType::KeyboardButton(KeyCode::S), Some(-1.0))
    .add(
        InputAxisType::GamepadAxis(GamepadAxisType::LeftStickY),
        None,
    );
```

### Spawn entity with InputID
```rust
fn spawn_player(
    mut commands: Commands,
    mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>,
) {
    commands.spawn().insert_bundle(PlayerBundle {
        player: Player {},
        input_id: input_bindings.create_input_id(InputType::Editor),
    });
}
```

### Hanle input
```rust
fn process_player_input(
    input_bindings: Res<UserInputHandle<InputType, Bindings>>,
    query: Query<&InputID>,
) {
    query.for_each_mut(|input_component| {
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            if let Some(value) = input_handle.get_axis_value(Bindings::Movement(MovementInput::Forward))
            {}

            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(HotkeysInput::Test))
            {}
        }
    });
}
```

### Other Functions
Create new input id for handle it from input_bindings
```rust
input_bindings.create_input_id(InputType::Editor)
```
Switch to new input bindings set for InputID
```rust
input_bindings.switch_input(&component, InputType::Editor);
```
Remove unused input from input_bindings
```rust
input_bindings.stop_input_tracking(&component);
```
Take current input source(Mouse, Keyboard, Gamepad). Could be used for game widgets, when you want to add button icon to it
```rust
input_bindings.get_input_source()
```
Take mouse position and delta
```rust
input_bindings.get_mouse_postion();
input_bindings.get_mouse_delta();
```
Lock or unlock mouse to window
```rust
input_bindings.lock_mouse();
input_bindings.unlock_mouse();
```
Create input handle for InputID, take current input type for InputID
```rust
if let Some(input_handle) = input_bindings.to_handle(input_component){
    let current_input_type = input_handle.get_input_type();
}
```
Handle multiple gamepads provided by InputID number. First created InputID has id equal 0, second equal 1. And first connected gamepad also will have id equal 0, second equal 1. InputID limited only by u8 numbers.

### Examples
See examples/common.rs for more information

### Limitations
Currently, plugin didn't support touch inputs, because posssibly it should have gestures processor and algorythm for simply generate gesture and handle it.
Also, gamepad values now filtered by abs(value) > 0.1, because bevy couldn't load and write settings to config right now with stable api
