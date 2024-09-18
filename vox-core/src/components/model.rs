use std::sync::Arc;

use bevy_ecs::component::Component;

use crate::{render::mesh::Mesh, Model};

#[derive(Component)]
pub struct ModelComponent {
    pub model: Arc<Model>,
}

impl From<Arc<Model>> for ModelComponent {
    fn from(value: Arc<Model>) -> Self {
        Self {
            model: value,
        }
    }
}

impl From<Mesh> for ModelComponent {
    fn from(value: Mesh) -> Self {
        let model = Model {
            meshes: vec![value],
            materials: vec![],
            name: String::from("Model from Mesh"),
        };

        let model = Arc::new(model);
        Self::from(model)
    }
}
