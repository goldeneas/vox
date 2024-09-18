use bevy_ecs::bundle::Bundle;

use crate::{components::{model::ModelComponent, transform::TransformComponent}, InstanceData};

#[derive(Bundle)]
pub struct Object {
    pub model: ModelComponent,
    pub transform: TransformComponent,
}
