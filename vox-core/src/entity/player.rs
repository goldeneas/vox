use bevy_ecs::prelude::*;

use crate::components::PositionComponent;
use crate::entity::vox_entity::VoxEntity;

pub struct PlayerEntity {
    pub id: Entity,
}

impl VoxEntity for PlayerEntity {
    fn new(world: &mut World) -> Self {
        let id = world.spawn((
                PositionComponent { x: 0.0, y: 0.0, z: 0.0 },
            )).id();

        Self {
            id,
        }
    }
}
