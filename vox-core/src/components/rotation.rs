use bevy_ecs::component::Component;

#[derive(Component)]
pub struct RotationComponent {
    pub quaternion: cgmath::Quaternion<f32>,
}

