use bevy_ecs::prelude::*;
use winit::event::ElementState;

#[derive(Resource, Default, Debug)]
pub struct InputRes {
    pub forward: KeyState,
    pub right: KeyState,
    pub left: KeyState,
    pub backward: KeyState,
}

#[derive(Default, Debug)]
pub struct KeyState {
    pub is_pressed: bool,
    pub is_released: bool,
}

impl From<&ElementState> for KeyState {
    fn from(item: &ElementState) -> Self {
        KeyState {
            is_pressed: item.is_pressed(),
            is_released: !item.is_pressed(),
        }
    }
}
