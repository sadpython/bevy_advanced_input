use bevy::{input::ElementState, prelude::*};
use bevy_advanced_input::prelude::*;

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
    Forward,
    Right,
    Up,
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step 2.2: More enums
enum CameraInput {
    Yaw,
    Pitch,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)] //* Step2.3: And even more enums
enum HotkeysInput {
    Test,
}

#[derive(Debug, Component)]
struct Player {} //* Step 3: Just player placeholder, you could skip this

#[derive(Bundle)]
struct PlayerBundle {
    //* Step 3.1: Player bundle also optional, you need only InputComponent on your entity
    player: Player,
    input_id: InputId,
}

#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
enum Labels {
    SetupInputs,
    SpawnPlayer,
    Input,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputBindingPlugin::<InputType, Bindings>::default()) //* Step 4: Add plugin with your InputType and BindingType
        .add_startup_system(setup_input.label(Labels::SetupInputs)) //* Step 5: Create setup input system
        .add_startup_system(spawn_player.label(Labels::SpawnPlayer)) //* Step 6: Spawn player
        .add_system(
            process_player_input
                .after(InputSystemLabels::RawInput)
                .label(Labels::Input),
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
    mut query: Query<&InputId>,
) {
    //* If need to track last input type - use input_bindings.get_input_source()
    //* It could be Keyboard, Mouse or Gamepad now, and could be used for game widgets, when you want to add button icon to it
    query.for_each_mut(|input_component| {
        //* Get input handle
        if let Some(input_handle) = input_bindings.to_handle(input_component) {
            //* Now we can call input_handle.get_axis_value() or input_handle.get_key_state() for track bindigs, see examples below
            //* Also, we can get current InputType from input_handle.get_input_type function, because we can!
            //* And finally - we can switch input type for out input component input_bindings.swich_input(component, type)
            //* If you need mouse position or delta(last frame) call input_bindings.get_mouse_postion() or input_bindings.get_mouse_delta()
            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::Right))
            {
                println!("Bindings::Movement(MovementInput::Right) -> {}", value);
            }
            if let Some(value) = input_handle.get_axis_value(Bindings::Movement(MovementInput::Up))
            {
                println!("Bindings::Movement(MovementInput::Up) -> {}", value);
            }

            if let Some(value) =
                input_handle.get_axis_value(Bindings::Movement(MovementInput::Forward))
            {
                println!("Bindings::Movement(MovementInput::Forward) -> {}", value);
            }

            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::Yaw)) {
                println!("Bindings::Camera(CameraInput::Yaw) -> {}", value);
            }
            if let Some(value) = input_handle.get_axis_value(Bindings::Camera(CameraInput::Pitch)) {
                println!("Bindings::Camera(CameraInput::Pitch) -> {}", value);
            }
            if let Some(value) = input_handle.get_key_state(Bindings::Hotkeys(HotkeysInput::Test)) {
                println!(
                    "Bindings::Hotkeys(HotkeysInput::Test) -> {}",
                    match value {
                        ElementState::Pressed => "Pressed",
                        ElementState::Released => "Released",
                    }
                );
            }
        }
    });
}

fn setup_input(mut input_bindings: ResMut<UserInputHandle<InputType, Bindings>>) {
    //* If you didn't have a config loader, you could setup it right now
    let mut config: InputConfig<Bindings> = InputConfig::new();
    //* Rebind default axis value
    config.rebind_default_value(InputAxisType::KeyboardButton(KeyCode::S), -1.0);
    config.rebind_default_value(InputAxisType::KeyboardButton(KeyCode::A), -1.0);
    config.rebind_default_value(InputAxisType::KeyboardButton(KeyCode::Q), -1.0);
    config.rebind_default_value(
        InputAxisType::GamepadButton(GamepadButtonType::LeftTrigger2),
        -1.0,
    );

    //* Or just swap axises
    config.rebind_axis(
        InputAxisType::KeyboardButton(KeyCode::Key0),
        InputAxisType::KeyboardButton(KeyCode::Key1),
    );

    let mut set = UserInputSet::new(); //* Create InputSet at first

    //* And then bind your bindings enum to keys, you could use from 1 to n keys in keyset
    //* Optionally - call enable_repeat_all_for_reactivation(), so you need release all of pressed keys and press it again for toggle binding
    //* Keys provide "Pressed", "Released" or None state if not changed last tick
    set.begin_key(Bindings::Hotkeys(HotkeysInput::Test))
        .add(&[
            InputAxisType::KeyboardButton(KeyCode::Q),
            InputAxisType::KeyboardButton(KeyCode::W),
        ])
        .enable_repeat_all_for_reactivation();

    //* Or bind your bindings enum to axis
    set.begin_axis(Bindings::Movement(MovementInput::Forward))
        .add(InputAxisType::KeyboardButton(KeyCode::W))
        .add(InputAxisType::KeyboardButton(KeyCode::S))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickY));

    set.begin_axis(Bindings::Movement(MovementInput::Right))
        .add(InputAxisType::KeyboardButton(KeyCode::A))
        .add(InputAxisType::KeyboardButton(KeyCode::D))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::LeftStickX));

    set.begin_axis(Bindings::Movement(MovementInput::Up))
        .add(InputAxisType::KeyboardButton(KeyCode::Q))
        .add(InputAxisType::KeyboardButton(KeyCode::E))
        .add(InputAxisType::GamepadButton(
            GamepadButtonType::RightTrigger2,
        ))
        .add(InputAxisType::GamepadButton(
            GamepadButtonType::LeftTrigger2,
        ));

    set.begin_axis(Bindings::Camera(CameraInput::Yaw))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::X))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::RightStickX));

    set.begin_axis(Bindings::Camera(CameraInput::Pitch))
        .add(InputAxisType::MouseAxisDiff(MouseAxisType::Y))
        .add(InputAxisType::GamepadAxis(GamepadAxisType::RightStickY));

    //* Add your input set to bindigs with specified game InputType
    input_bindings.add_input(InputType::Editor, set);

    //* And last step - apply your config. It will be applyed to all of your input sets, and you could apply config many times if you need
    //* change game settings, for example. Config didn't cached, so
    input_bindings.apply_config(&config);
}
