use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
