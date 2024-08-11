use bevy_ecs::component::Component;
use cgmath::Point3;

#[derive(Component)]
pub struct PositionComponent {
    pub position: Point3<f32>,
}
