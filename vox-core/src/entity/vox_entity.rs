use bevy_ecs::prelude::*;
use winit::event::WindowEvent;

#[allow(unused_variables)]
pub trait VoxEntity {
    fn new(world: &mut World) -> Self;
    fn input(&mut self, event: &WindowEvent) -> bool {}
    fn update(&mut self) {}
}
