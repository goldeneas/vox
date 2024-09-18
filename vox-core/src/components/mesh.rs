use bevy_ecs::component::Component;

use crate::render::mesh::Mesh;

#[derive(Component)]
pub struct MeshComponent {
    pub mesh: Mesh,
}
