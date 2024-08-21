use bevy_ecs::schedule::{IntoSystemConfigs, SystemConfigs};

use crate::resources::game_state::GameState;

pub trait Screen {
    fn game_state(&self) -> &GameState;

    fn start(&mut self) {}

    fn start_systems(&self) -> Option<SystemConfigs> { None }
    fn ui_systems(&self) -> Option<SystemConfigs> { None }
    fn draw_systems(&self) -> Option<SystemConfigs> { None }
    fn update_systems(&self) -> Option<SystemConfigs> { None }

    fn to_systems<M>(&self,
        systems: impl IntoSystemConfigs<M>, 
    ) -> Option<SystemConfigs> where Self: Sized {
        Some(systems.into_configs())
    }
}
