use bevy::prelude::*;

use bevy_advanced_input::{
    component::InputID,
    plugin::InputBindingPlugin,
    user_input::{InputAxisType, MouseAxisType, UserInputHandle, UserInputSet},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step 1: create input states for your game
enum InputType {
    Editor,
    MainMenu,
    Game,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step 2: At second: create binginds enum for all game inputs: main menu, camera, movement
enum Bindings {
    Hotkeys(HotkeysInput),
    Movement(MovementInput),
    Camera(CameraInput),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step 2.1: And create enums for all of these situations
enum MovementInput {
    forward,
    right,
    up,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step 2.2: More enums
enum CameraInput {
    yaw,
    pitch,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step2.3: And even more enums
enum HotkeysInput {
    test,
}

#[derive(Debug)]
struct Player {} //* Step 3: Just player placeholder, you could skip this

#[derive(Bundle)]
struct PlayerBundle {
    //* Step 3.1: Player bundle also optional, you need only InputComponent on your entity
    player: Player,
    input_id: InputID,
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputBindingPlugin::<InputType, Bindings>::default()) //* Step 4: Add plugin with your InputType and BindingType
        .add_startup_system(setup_input.system().label("setup_inputs")) //* Step 5: Create setup input system
        .add_startup_system(spawn_player.system().label("spawn_player")) //* Step 6: Spawn player
        .add_system(
            process_player_input
                .system()
                .after("raw_input")
                .label("input"),
        ) //* Step 7: create process input system, it must be runned after out "raw_input" system
        .run();
}

fn spawn_player(
    mut commands: Commands,
    mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>,
) {
    //* When we spawn our player - use input_bindings.create_input_component(InputType) to create input component
    //* It automaticly create InputID with id inside it, technicly it's just a number
    commands.spawn().insert_bundle(PlayerBundle {
        player: Player {},
        input_id: input_bindings.create_input_id(InputType::Editor),
    });
}

fn process_player_input(
    input_bindings: Res<UserInputHandle<InputType, Bindings>>,
    query: Query<&InputID>,
) {
    //* If need to track last input type - use input_bindings.get_input_source()
    //* It could be Keyboard, Mouse or Gamepad now, and could be used for game widgets, when you want to add button icon to it
    query.for_each_mut(|input_component| {
        //* Get input handle
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            //* Now we can call input_handle.get_axis_value() or input_handle.get_key_state() for track bindigs, see examples below
            //* Also, we can get current InputType from input_handle.get_input_type function, because we can!
            //* And finally - we can switch input type for out input component input_bindings.swich_input(component, type)
            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::right))
            {}
            if let Some(value) = input_handle.get_axis_value(Bindings::Movement(MovementInput::up))
            {
            }

            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::forward))
            {
            }

            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::yaw)) {}
            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::pitch)) {
            }
            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(HotkeysInput::test)) {
            }
        }
    });
}

fn setup_input(mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>) {
    let mut set = UserInputSet::new(); //* Create InputSet at first

    //* And then bind your bindings enum to keys, you could use from 1 to n keys in keyset
    //* Optionally - call enable_repeat_all_for_reactivation(), so you need release all of pressed keys and press it again for toggle binding
    //* Keys provide "Pressed", "Released" or None state if not changed last tick
    set.begin_key(Bindings::Hotkeys(HotkeysInput::test))
        .add(&[
            InputAxisType::KeyboardButton(KeyCode::Q),
            InputAxisType::KeyboardButton(KeyCode::W),
        ])
        .enable_repeat_all_for_reactivation();

    //* Or bind your bindings enum to axis, common way to binding your inputs
    //* Axis provide a float value of last pressed button or axis, or None if not pressed
    //* Axis result value calculate from:
    //* axis_value(default equal 1, but could be overwrited by input like GamepadAxis, which provide values from -1.0 to 1.0)
    //* default_value(default equal 1, but could be overwited by default_value when add binding)
    //* Final axis value is axis_value * default_value
    set.begin_axis(Bindings::Movement(MovementInput::forward))
        .add(InputAxisType::KeyboardButton(KeyCode::W), Some(1.0))
        .add(InputAxisType::KeyboardButton(KeyCode::S), Some(-1.0))
        .add(
            InputAxisType::GamepadAxis(GamepadAxisType::LeftStickY),
            None,
        );

    set.begin_axis(Bindings::Movement(MovementInput::right))
        .add(InputAxisType::KeyboardButton(KeyCode::A), Some(-1.0))
        .add(InputAxisType::KeyboardButton(KeyCode::D), Some(1.0))
        .add(
            InputAxisType::GamepadAxis(GamepadAxisType::LeftStickX),
            None,
        );

    set.begin_axis(Bindings::Movement(MovementInput::up))
        .add(InputAxisType::KeyboardButton(KeyCode::Q), Some(-1.0))
        .add(InputAxisType::KeyboardButton(KeyCode::E), Some(1.0))
        .add(
            InputAxisType::GamepadButton(GamepadButtonType::RightTrigger2),
            None,
        )
        .add(
            InputAxisType::GamepadButton(GamepadButtonType::LeftTrigger2),
            Some(-1.0),
        );

    set.begin_axis(Bindings::Camera(CameraInput::yaw))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::X), None)
        .add(
            InputAxisType::GamepadAxis(GamepadAxisType::RightStickX),
            None,
        );

    set.begin_axis(Bindings::Camera(CameraInput::pitch))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::Y), None)
        .add(
            InputAxisType::GamepadAxis(GamepadAxisType::RightStickY),
            None,
        );

    //* And last step - add your input set to bindigs with specified game InputType
    input_bindings.add_input(InputType::Editor, set);
}
