use std::{collections::hash_map::Entry, hash::Hash};

use bevy::{
    input::ElementState,
    math::Vec2,
    prelude::{
        Gamepad, GamepadAxisType, GamepadButtonType, GamepadEventType, KeyCode, MouseButton,
    },
    utils::HashMap,
};

use crate::common::InsertOrGet;

use super::input_id::InputID;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum InputState {
    Released,
    ShouldBeActivated,
    ShouldBeDeactvated,
    Pressed,
}

#[derive(Clone, Debug)]
pub struct InputKeyset {
    pub(crate) state: InputState,
    pub(crate) activated_keys_num: usize,
    pub(crate) keys_state: HashMap<InputAxisType, ElementState>,
    pub(crate) repeat_all_for_activate: bool,
}

impl InputKeyset {
    pub fn new(keyset: &[InputAxisType], repeat_all_for_activate: bool) -> Self {
        let mut set = Self {
            state: InputState::Released,
            activated_keys_num: 0,
            keys_state: HashMap::default(),
            repeat_all_for_activate,
        };
        for key in keyset {
            set.keys_state.insert(key.clone(), ElementState::Released);
        }
        set
    }

    pub fn update_key_state(&mut self, key: InputAxisType, new_state: ElementState) {
        if let Some(val) = self.keys_state.get_mut(&key) {
            if *val != new_state {
                *val = new_state;
                match new_state {
                    ElementState::Pressed => {
                        self.activated_keys_num += 1;
                    }
                    ElementState::Released => {
                        if self.repeat_all_for_activate {
                            self.activated_keys_num = 0;
                        } else {
                            self.activated_keys_num -= 1;
                        }
                    }
                }
                if self.activated_keys_num == self.keys_state.len() {
                    if self.state != InputState::Pressed {
                        self.state = InputState::ShouldBeActivated;
                    }
                } else if self.state == InputState::Pressed {
                    self.state = InputState::ShouldBeDeactvated;
                }
            }
        }
    }

    pub(crate) fn update_state(&mut self) {
        match self.state {
            InputState::Released => {}
            InputState::ShouldBeActivated => {
                self.state = InputState::Pressed;
            }
            InputState::ShouldBeDeactvated => {
                self.state = InputState::Released;
            }
            InputState::Pressed => {}
        }
    }
}

impl Default for InputKeyset {
    fn default() -> Self {
        Self::new(&[], false)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum MouseAxisType {
    X,
    Y,
    Wheel,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
#[allow(dead_code)]
pub enum InputAxisType {
    KeyboardButton(KeyCode),
    MouseButton(MouseButton),
    GamepadButton(GamepadButtonType),
    MouseAxis(MouseAxisType),
    MouseAxisDiff(MouseAxisType),
    GamepadAxis(GamepadAxisType),
    GamepadAxisDiff(GamepadAxisType),
}

#[derive(Clone, Debug)]
pub struct InputAxisSet {
    pub(crate) state: InputState,
    pub(crate) axises: HashMap<InputAxisType, Option<f32>>,
    pub(crate) active_axis_types: Vec<InputAxisType>,
    pub(crate) out_value: Option<f32>,
}

impl InputAxisSet {
    pub fn new(axises: HashMap<InputAxisType, Option<f32>>) -> Self {
        Self {
            state: InputState::Released,
            axises,
            active_axis_types: Vec::new(),
            out_value: None,
        }
    }
    #[allow(dead_code)]
    pub fn add_axis(&mut self, axis_type: InputAxisType, axis_value: Option<f32>) {
        self.axises.insert(axis_type, axis_value);
    }

    pub fn update_axis_state(
        &mut self,
        axis_type: InputAxisType,
        new_state: ElementState,
        value: Option<f32>,
    ) {
        if let Entry::Occupied(entry) = self.axises.entry(axis_type.clone()) {
            match new_state {
                ElementState::Pressed => {
                    let mut should_update_value = false;
                    if self.active_axis_types.iter().any(|elem| *elem == axis_type) {
                        if *self.active_axis_types.last().unwrap() == axis_type {
                            should_update_value = true;
                        }
                    } else {
                        self.active_axis_types.push(axis_type);
                        should_update_value = true;
                        if self.active_axis_types.len() == 1 {
                            self.state = InputState::ShouldBeActivated;
                        }
                    }

                    if should_update_value {
                        let default_value = entry.get().unwrap_or(1.0);
                        let new_value = value.unwrap_or(1.0);
                        self.out_value = Some(default_value * new_value);
                    }
                }
                ElementState::Released => {
                    if let Some(index) = self
                        .active_axis_types
                        .iter()
                        .position(|elem| *elem == axis_type)
                    {
                        self.active_axis_types.remove(index);

                        if let Some(last_axis) = self.active_axis_types.last() {
                            let axis = last_axis.clone();
                            self.update_axis_state(axis, ElementState::Pressed, None);
                        } else {
                            self.out_value = None;
                            self.state = InputState::ShouldBeDeactvated;
                        }
                    }
                }
            }
        }
    }

    fn get_value(&self) -> Option<f32> {
        self.out_value
    }

    pub(crate) fn update_state(&mut self) {
        match self.state {
            InputState::Released => {}
            InputState::ShouldBeActivated => {
                self.state = InputState::Pressed;
            }
            InputState::ShouldBeDeactvated => {
                self.state = InputState::Released;
                self.active_axis_types.clear();
            }
            InputState::Pressed => {}
        }
    }
}
#[derive(Clone)]
pub struct UserInputSet<Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    name_to_keyset: HashMap<Key, InputKeyset>,
    name_to_axisset: HashMap<Key, InputAxisSet>,
    last_gamepad_axis_value: HashMap<GamepadAxisType, f32>,
}

pub struct AxisSetBuilder<'a, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    axises: HashMap<InputAxisType, Option<f32>>,
    name: Key,
    owner_set: &'a mut UserInputSet<Key>,
}

impl<'a, Key> AxisSetBuilder<'a, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn add(&mut self, axis_type: InputAxisType, default_value: Option<f32>) -> &mut Self {
        self.axises.insert(axis_type, default_value);
        self
    }

    fn finish(&mut self) {
        self.owner_set.add_axisset(self.name, self.axises.clone());
    }
}

impl<Key> Drop for AxisSetBuilder<'_, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn drop(&mut self) {
        self.finish();
    }
}

pub struct KeySetBuilder<'a, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    axises: Vec<InputAxisType>,
    name: Key,
    owner_set: &'a mut UserInputSet<Key>,
    repeat_all_for_reactivate: bool,
}

impl<'a, Key> KeySetBuilder<'a, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn add(&mut self, keys: &[InputAxisType]) -> &mut Self {
        let mut vec = keys.to_vec();
        self.axises.append(&mut vec);
        self
    }

    pub fn enable_repeat_all_for_reactivation(&mut self) -> &mut Self {
        self.repeat_all_for_reactivate = true;
        self
    }

    fn finish(&mut self) {
        self.owner_set
            .add_keyset(self.name, &self.axises, self.repeat_all_for_reactivate);
    }
}

impl<Key> Drop for KeySetBuilder<'_, Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn drop(&mut self) {
        self.finish();
    }
}

impl<Key> UserInputSet<Key>
where
    Key: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            name_to_keyset: HashMap::default(),
            name_to_axisset: HashMap::default(),
            last_gamepad_axis_value: HashMap::default(),
        }
    }

    pub fn begin_key(&mut self, name: Key) -> KeySetBuilder<Key> {
        KeySetBuilder {
            axises: Vec::new(),
            name: name,
            owner_set: self,
            repeat_all_for_reactivate: false,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn add_keyset(
        &mut self,
        name: Key,
        keyset: &Vec<InputAxisType>,
        repeat_all_for_activate: bool,
    ) {
        self.name_to_keyset
            .insert(name, InputKeyset::new(keyset, repeat_all_for_activate));
    }

    #[allow(dead_code)]
    pub fn begin_axis(&mut self, name: Key) -> AxisSetBuilder<Key> {
        AxisSetBuilder {
            axises: HashMap::default(),
            name: name,
            owner_set: self,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn add_axisset(&mut self, name: Key, axises: HashMap<InputAxisType, Option<f32>>) {
        self.name_to_axisset.insert(name, InputAxisSet::new(axises));
    }

    pub fn get_axis_value(&self, name: Key) -> Option<f32> {
        if let Some(val) = self.name_to_axisset.get(&name) {
            return val.get_value();
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_key_state(&self, name: Key) -> Option<ElementState> {
        if let Some(val) = self.name_to_keyset.get(&name) {
            if val.state == InputState::ShouldBeActivated {
                return Some(ElementState::Pressed);
            } else if val.state == InputState::ShouldBeDeactvated {
                return Some(ElementState::Released);
            }
        }
        None
    }

    pub(crate) fn change_key_state(&mut self, key_type: InputAxisType, state: ElementState) {
        for (_, keyset) in self.name_to_keyset.iter_mut() {
            keyset.update_key_state(key_type.clone(), state);
        }
    }

    pub(crate) fn change_axis_state(
        &mut self,
        axis_type: InputAxisType,
        state: ElementState,
        value: Option<f32>,
    ) {
        for (_, keyset) in self.name_to_axisset.iter_mut() {
            keyset.update_axis_state(axis_type.clone(), state, value);
        }
    }

    pub(crate) fn update_states(&mut self) {
        for (_, keyset) in self.name_to_keyset.iter_mut() {
            keyset.update_state();
        }
        for (_, axisset) in self.name_to_axisset.iter_mut() {
            axisset.update_state();
        }
    }
}

impl<Key> Default for UserInputSet<Key>
where
    Key: Eq + PartialEq + Clone + Copy + Hash + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
pub enum TouchState {
    Pressed,
    Released,
    Moved,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum InputSource {
    Keyboard,
    Mouse,
    Gamepad,
}

pub struct UserInputHandle<InputType, BindingType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    mouse_position: Option<Vec2>,
    mouse_delta: Option<Vec2>,
    mouse_moved_this_tick: bool,
    mouse_wheel_moved_this_tick: bool,
    input_id_to_inputset: HashMap<u8, UserInputSet<BindingType>>,
    input_id_to_input_type: HashMap<u8, InputType>,
    available_sets: HashMap<InputType, UserInputSet<BindingType>>,
    last_input_source: Option<InputSource>,
}

impl<InputType, BindingType> Default for UserInputHandle<InputType, BindingType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<InputType, BindingType> UserInputHandle<InputType, BindingType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            mouse_position: None,
            mouse_delta: None,
            mouse_moved_this_tick: false,
            mouse_wheel_moved_this_tick: false,
            input_id_to_inputset: HashMap::default(),
            input_id_to_input_type: HashMap::default(),
            available_sets: HashMap::default(),
            last_input_source: None,
        }
    }
    pub(crate) fn process_keyboard_key(&mut self, key: KeyCode, new_state: ElementState) {
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            player_set.change_key_state(InputAxisType::KeyboardButton(key), new_state);
            player_set.change_axis_state(InputAxisType::KeyboardButton(key), new_state, None);
        }
        self.last_input_source = Some(InputSource::Keyboard);
    }
    pub(crate) fn process_mouse(&mut self, current_position: Vec2, delta_position: Vec2) {
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            player_set.change_axis_state(
                InputAxisType::MouseAxis(MouseAxisType::X),
                ElementState::Pressed,
                Some(current_position.x),
            );
            player_set.change_axis_state(
                InputAxisType::MouseAxis(MouseAxisType::Y),
                ElementState::Pressed,
                Some(current_position.y),
            );
            player_set.change_axis_state(
                InputAxisType::MouseAxisDiff(MouseAxisType::X),
                ElementState::Pressed,
                Some(delta_position.x),
            );
            player_set.change_axis_state(
                InputAxisType::MouseAxisDiff(MouseAxisType::Y),
                ElementState::Pressed,
                Some(delta_position.y),
            );
        }

        self.mouse_position = Some(current_position);
        self.mouse_delta = Some(delta_position);

        self.mouse_moved_this_tick = true;
        self.last_input_source = Some(InputSource::Mouse);
    }
    pub(crate) fn process_mouse_button(&mut self, button: MouseButton, new_state: ElementState) {
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            player_set.change_key_state(InputAxisType::MouseButton(button), new_state);
            player_set.change_axis_state(InputAxisType::MouseButton(button), new_state, None);
        }
        self.last_input_source = Some(InputSource::Mouse);
    }
    pub(crate) fn process_mouse_wheel(&mut self, delta: Vec2) {
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            player_set.change_key_state(
                InputAxisType::MouseAxis(MouseAxisType::Wheel),
                ElementState::Pressed,
            );
            player_set.change_key_state(
                InputAxisType::MouseAxisDiff(MouseAxisType::Wheel),
                ElementState::Pressed,
            );
            player_set.change_axis_state(
                InputAxisType::MouseAxis(MouseAxisType::Wheel),
                ElementState::Pressed,
                Some(delta.y),
            );
            player_set.change_axis_state(
                InputAxisType::MouseAxisDiff(MouseAxisType::Wheel),
                ElementState::Pressed,
                Some(delta.y),
            );
        }
        self.mouse_wheel_moved_this_tick = true;
        self.last_input_source = Some(InputSource::Mouse);
    }

    //TODO: rewrite
    // #[allow(dead_code)]
    // pub(crate) fn process_touch(&mut self, finger: u8, position: Vec2, state: TouchPhase) {
    //     //let mut last_touches = input_set.last_touch_indexes;
    //     match state {
    //         TouchPhase::Started => {
    //             for (_, player_set) in self.current_set.iter_mut() {
    //                 player_set.change_key_state(
    //                     InputAxisType::TouchFinger(finger),
    //                     ElementState::Pressed,
    //                 );
    //                 player_set.touch_to_position.insert(finger, position);
    //                 player_set.last_touch_indexes.push(finger);
    //             }
    //         }
    //         TouchPhase::Moved => {
    //             for (_, player_set) in self.current_set.iter_mut() {
    //                 player_set.change_axis_state(
    //                     InputAxisType::TouchAxis(TouchAxisType::X),
    //                     ElementState::Pressed,
    //                     Some(position.x),
    //                 );
    //                 player_set.change_axis_state(
    //                     InputAxisType::TouchAxis(TouchAxisType::Y),
    //                     ElementState::Pressed,
    //                     Some(position.y),
    //                 );
    //                 let position_diff = match player_set.touch_to_position.entry(finger) {
    //                     Entry::Occupied(mut exists) => {
    //                         let last_position = exists.insert(position);
    //                         position - last_position
    //                     }
    //                     Entry::Vacant(empty) => {
    //                         empty.insert(position);
    //                         Vec2::new(0.0, 0.0)
    //                     }
    //                 };
    //                 player_set.change_axis_state(
    //                     InputAxisType::TouchAxisDiff(TouchAxisType::X),
    //                     ElementState::Pressed,
    //                     Some(position_diff.x),
    //                 );
    //                 player_set.change_axis_state(
    //                     InputAxisType::TouchAxisDiff(TouchAxisType::Y),
    //                     ElementState::Pressed,
    //                     Some(position_diff.y),
    //                 );
    //             }
    //         }
    //         TouchPhase::Ended => {
    //             for (_, player_set) in self.current_set.iter_mut() {
    //                 player_set.change_key_state(
    //                     InputAxisType::TouchFinger(finger),
    //                     ElementState::Released,
    //                 );
    //                 player_set.touch_to_position.remove(&finger);
    //                 let index = player_set
    //                     .last_touch_indexes
    //                     .iter()
    //                     .position(|elem| *elem == finger)
    //                     .unwrap();
    //                 player_set.last_touch_indexes.remove(index);
    //             }
    //         }
    //         TouchPhase::Cancelled => {
    //             for (_, player_set) in self.current_set.iter_mut() {
    //                 player_set.change_key_state(
    //                     InputAxisType::TouchFinger(finger),
    //                     ElementState::Released,
    //                 );
    //                 player_set.touch_to_position.remove(&finger);
    //                 let index = player_set
    //                     .last_touch_indexes
    //                     .iter()
    //                     .position(|elem| *elem == finger)
    //                     .unwrap();
    //                 player_set.last_touch_indexes.remove(index);
    //             }
    //         }
    //     }
    // }

    #[allow(dead_code)]
    pub(crate) fn process_gamepad(&mut self, gamepad: Gamepad, event: GamepadEventType) {
        //TODO: Write connection logic
        match event {
            GamepadEventType::Connected => {}
            GamepadEventType::Disconnected => {}
            GamepadEventType::ButtonChanged(btn_type, value) => {
                let state = if value.abs() <= 0.1 {
                    ElementState::Released
                } else {
                    ElementState::Pressed
                };
                for (player_id, player_set) in self.input_id_to_inputset.iter_mut() {
                    if *player_id == gamepad.0 as u8 {
                        player_set.change_key_state(InputAxisType::GamepadButton(btn_type), state);
                        player_set.change_axis_state(
                            InputAxisType::GamepadButton(btn_type),
                            state,
                            Some(value),
                        );
                        break;
                    }
                }
                self.last_input_source = Some(InputSource::Gamepad);
            }
            GamepadEventType::AxisChanged(axis_type, value) => {
                let state = if value.abs() <= 0.1 {
                    ElementState::Released
                } else {
                    ElementState::Pressed
                };
                for (player_id, player_set) in self.input_id_to_inputset.iter_mut() {
                    if *player_id == gamepad.0 as u8 {
                        player_set.change_key_state(InputAxisType::GamepadAxis(axis_type), state);
                        player_set.change_axis_state(
                            InputAxisType::GamepadAxis(axis_type),
                            state,
                            Some(value),
                        );
                        let diff = match player_set.last_gamepad_axis_value.entry(axis_type) {
                            Entry::Occupied(mut exists) => {
                                let last_value = exists.insert(value);
                                value - last_value
                            }
                            Entry::Vacant(empty) => {
                                empty.insert(value);
                                0.0
                            }
                        };
                        player_set.change_axis_state(
                            InputAxisType::GamepadAxisDiff(axis_type),
                            state,
                            Some(diff),
                        );
                        break;
                    }
                }

                self.last_input_source = Some(InputSource::Gamepad);
            }
        }
    }

    #[allow(dead_code)]
    pub fn switch_input(&mut self, component: &'_ InputID, input_type: InputType) {
        let should_change_input =
            if let Some(exists_input) = self.input_id_to_input_type.get(&component.id) {
                *exists_input == input_type
            } else {
                true
            };
        if should_change_input {
            if let Entry::Occupied(set) = self.available_sets.entry(input_type) {
                self.input_id_to_inputset
                    .insert(component.id, set.get().clone());
                self.input_id_to_input_type.insert(component.id, input_type);
            }
        }
    }

    #[allow(dead_code)]
    pub fn add_input(&mut self, input_type: InputType, input_set: UserInputSet<BindingType>) {
        let map = self.available_sets.insert_or_get(input_type);
        *map = input_set;
    }

    fn update_states(&mut self) {
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            player_set.update_states();
        }
    }

    pub(crate) fn finish_processing(&mut self) {
        self.update_states();
        for (_, player_set) in self.input_id_to_inputset.iter_mut() {
            if !self.mouse_moved_this_tick {
                player_set.change_axis_state(
                    InputAxisType::MouseAxis(MouseAxisType::X),
                    ElementState::Released,
                    None,
                );
                player_set.change_axis_state(
                    InputAxisType::MouseAxis(MouseAxisType::Y),
                    ElementState::Released,
                    None,
                );
                player_set.change_axis_state(
                    InputAxisType::MouseAxisDiff(MouseAxisType::X),
                    ElementState::Released,
                    None,
                );
                player_set.change_axis_state(
                    InputAxisType::MouseAxisDiff(MouseAxisType::Y),
                    ElementState::Released,
                    None,
                );
            };
            if !self.mouse_wheel_moved_this_tick {
                player_set.change_axis_state(
                    InputAxisType::MouseAxis(MouseAxisType::Wheel),
                    ElementState::Released,
                    None,
                );
                player_set.change_axis_state(
                    InputAxisType::MouseAxisDiff(MouseAxisType::Wheel),
                    ElementState::Released,
                    None,
                );
                player_set.change_key_state(
                    InputAxisType::MouseAxis(MouseAxisType::Wheel),
                    ElementState::Released,
                );
                player_set.change_key_state(
                    InputAxisType::MouseAxisDiff(MouseAxisType::Wheel),
                    ElementState::Released,
                );
            };
        }

        self.mouse_moved_this_tick = false;

        self.mouse_wheel_moved_this_tick = false;

        self.mouse_delta = None;
    }

    pub fn to_handle(
        &self,
        component: &'_ InputID,
    ) -> Option<InputHandle<'_, BindingType, InputType>> {
        if let (Some(input_set), Some(input_type)) = (
            self.input_id_to_inputset.get(&component.id),
            self.input_id_to_input_type.get(&component.id),
        ) {
            return Some(InputHandle {
                input_set,
                input_type,
            });
        }
        None
    }

    pub fn create_input_id(&mut self, input_type: InputType) -> InputID {
        let component = InputID::default();
        self.switch_input(&component, input_type);
        component
    }

    #[allow(dead_code)]
    pub fn stop_input_tracking(&mut self, component: &'_ InputID) {
        self.input_id_to_inputset.remove(&component.id);
        self.input_id_to_input_type.remove(&component.id);
    }

    #[allow(dead_code)]
    pub fn get_input_source(&self) -> Option<InputSource> {
        self.last_input_source
    }

    #[allow(dead_code)]
    pub fn get_mouse_postion(&self) -> Option<Vec2> {
        self.mouse_position
    }

    #[allow(dead_code)]
    pub fn get_mouse_delta(&self) -> Option<Vec2> {
        self.mouse_delta
    }
}

pub struct InputHandle<'a, BindingType, InputType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    input_set: &'a UserInputSet<BindingType>,
    input_type: &'a InputType,
}

impl<BindingType, InputType> InputHandle<'_, BindingType, InputType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    #[allow(dead_code)]
    pub fn get_axis_value(&self, name: BindingType) -> Option<f32> {
        self.input_set.get_axis_value(name)
    }

    #[allow(dead_code)]
    pub fn get_key_state(&self, name: BindingType) -> Option<ElementState> {
        self.input_set.get_key_state(name)
    }

    #[allow(dead_code)]
    pub fn get_input_type(&self) -> &'_ InputType {
        self.input_type
    }
}
