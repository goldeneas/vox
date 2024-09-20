use std::sync::Arc;

use bevy_ecs::component::Component;

use crate::{render::mesh::Mesh, Model};

#[derive(Component)]
pub struct ModelComponent {
    pub model: Arc<Model>,
}
