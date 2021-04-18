# Bevy advanced inputs
This plugin provide functionality for bindig axises and keysets in bevy.

### Binding input
```rust
    let mut set = UserInputSet::new()

    set.begin_key(Bindings::Hotkeys(HotkeysInput::test))
        .add(&[
            InputAxisType::KeyboardButton(KeyCode::Q),
            InputAxisType::KeyboardButton(KeyCode::W),
        ])
        .enable_repeat_all_for_reactivation();

    set.begin_axis(Bindings::Movement(MovementInput::forward))
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
    //* If need to track last input type - use input_bindings.get_input_source()
    //* It could be Keyboard, Mouse or Gamepad now, and could be used for game widgets, when you want to add button icon to it
    query.for_each_mut(|input_component| {
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            if let Some(value) = input_handle.get_axis_value(Bindings::Movement(MovementInput::right))
            {}

            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::yaw)) 
            {}

            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(HotkeysInput::test)) 
            {}
        }
    });
}
```
### Examples
See examples/common.rs for more information

### Limitations
Currently, plugin didn't support touch inputs, because posssibly it should have gestures processor and algorythm for simply generate gesture and handle it.