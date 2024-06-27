use bevy_ecs::prelude::*;

#[allow(dead_code)]
pub trait VoxEntity {
    fn new(world: &mut World) -> Self;
}
