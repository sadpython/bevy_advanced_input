use bevy::{prelude::KeyCode, utils::HashMap};
use std::hash::Hash;

use crate::common::InsertOrGet;
use crate::user_input::InputAxisType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InputConfig<BindingType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub(crate) convert_pressed_key_to: HashMap<InputAxisType, InputAxisType>,
    axis_multiplyer: HashMap<BindingType, HashMap<InputAxisType, f32>>,
    common_axis_multiplyer: HashMap<InputAxisType, f32>,
}

impl<BindingType> InputConfig<BindingType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            convert_pressed_key_to: HashMap::default(),
            axis_multiplyer: HashMap::default(),
            common_axis_multiplyer: HashMap::default(),
        }
    }
}

impl<BindingType> Default for InputConfig<BindingType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<BindingType> InputConfig<BindingType>
where
    BindingType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    pub fn rebind_axis(&mut self, from: InputAxisType, to: InputAxisType) {
        self.convert_pressed_key_to.insert(from, to);
    }

    pub fn rebind_default_value(&mut self, input_axis: InputAxisType, modifier: f32) {
        self.common_axis_multiplyer.insert(input_axis, modifier);
    }

    pub fn get_default_value(&self, input_axis: &InputAxisType) -> f32 {
        if let Some(value) = self.common_axis_multiplyer.get(&input_axis) {
            return *value;
        }
        1.0
    }
}
