use std::hash::Hash;

use bevy::{
    input::{
        keyboard::KeyboardInput,
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
    },
    math::Vec2,
    prelude::{EventReader, GamepadEvent, ResMut},
    window::{CursorMoved, Windows},
};

use super::user_input::UserInputHandle;

//TODO: add touch support with gestures
#[allow(clippy::too_many_arguments)]
pub(crate) fn input_system<InputType: 'static, KeyType: 'static>(
    mut evr_keys: EventReader<KeyboardInput>,

    mut evr_cursor: EventReader<CursorMoved>,

    mut evr_motion: EventReader<MouseMotion>,

    mut evr_mousebtn: EventReader<MouseButtonInput>,

    mut evr_scroll: EventReader<MouseWheel>,

    mut evr_gamepad: EventReader<GamepadEvent>,
    mut user_input: ResMut<UserInputHandle<InputType, KeyType>>,
    mut windows: ResMut<Windows>,
) where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    KeyType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    user_input.finish_processing();
    // Keyboard input
    for ev in evr_keys.iter() {
        let state = ev.state;
        if let Some(key_code) = ev.key_code {
            user_input.process_keyboard_key(key_code, state);
        }
    }

    // Absolute cursor position (in window coordinates)
    for (abs_position, delta_position) in evr_cursor.iter().zip(evr_motion.iter()) {
        user_input.process_mouse(abs_position.position, delta_position.delta);
    }

    // Mouse buttons
    for ev in evr_mousebtn.iter() {
        user_input.process_mouse_button(ev.button, ev.state);
    }

    // scrolling (mouse wheel, touchpad, etc.)
    for ev in evr_scroll.iter() {
        user_input.process_mouse_wheel(Vec2::new(ev.x, ev.y));
    }

    // for ev in evr_touch.iter() {
    //     user_input.process_touch(ev.id as u8, ev.position, ev.phase);
    // }

    //Gamepad input
    for ev_gmp in evr_gamepad.iter() {
        user_input.process_gamepad(ev_gmp.0, ev_gmp.1.clone());
    }

    match user_input.mouse_lock_state {
        crate::user_input::MouseLockState::ShouldBeUnlocked => {
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }
        crate::user_input::MouseLockState::ShouldBeLocked => {
            let window = windows.get_primary_mut().unwrap();
            window.set_cursor_lock_mode(true);
            window.set_cursor_visibility(false);
        }
        _ => {}
    }

    user_input.update_states();
}
