use bevy_ecs::prelude::*;

#[derive(Resource, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

impl GameState {
    pub fn set(&mut self, state: GameState) {
        *self = state;
    }
}
