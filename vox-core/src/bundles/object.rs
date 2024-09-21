use bevy_ecs::bundle::Bundle;

use crate::Model;

#[derive(Bundle)]
pub struct Object {
    pub model: Model,
}
