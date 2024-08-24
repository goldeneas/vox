use bevy_ecs::{schedule::{IntoSystemConfigs, SystemConfigs}, world::World};

use crate::resources::game_state::GameState;

#[allow(unused_variables)]
pub trait Screen
where Self: Send + Sync + 'static {
    fn game_state(&self) -> GameState;

    fn start(&mut self, world: &mut World) {}
    fn ui(&mut self, world: &mut World) {}
    fn draw(&mut self, world: &mut World) {}
    fn update(&mut self, world: &mut World) {}

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
