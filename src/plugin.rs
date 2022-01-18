use std::hash::Hash;
use std::marker::PhantomData;

use super::{system::input_system, user_input::UserInputHandle};
use bevy::prelude::*;

pub struct InputBindingPlugin<InputType, KeyType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    KeyType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    phantom: PhantomData<InputType>,
    phantom2: PhantomData<KeyType>,
}

impl<InputType, KeyType> Default for InputBindingPlugin<InputType, KeyType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    KeyType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn default() -> Self {
        Self {
            phantom: PhantomData,
            phantom2: PhantomData,
        }
    }
}

impl<InputType: 'static, KeyType: 'static> Plugin for InputBindingPlugin<InputType, KeyType>
where
    InputType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
    KeyType: PartialEq + Eq + Hash + Copy + Clone + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.init_resource::<UserInputHandle<InputType, KeyType>>()
            .add_system(
                input_system::<InputType, KeyType>
                    .system()
                    .label("raw_input"),
            );
    }
}
