use std::sync::Arc;

use bevy_ecs::prelude::*;
use cgmath::{Quaternion, Zero};

use crate::{components::{model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent}, Model};

#[derive(Bundle)]
pub struct SingleEntity {
    position: PositionComponent,
    model: ModelComponent,
    instance: SingleInstanceComponent,
    rotation: RotationComponent,
}

impl SingleEntity {
    pub fn new(model: Arc<Model>) -> Self {
        Self {
            position: PositionComponent {
                position: (0.0, 0.0, 0.0).into() 
            },
            model: ModelComponent {
                model,
            },
            rotation: RotationComponent {
                quaternion: Quaternion::zero(),
            },
            instance: SingleInstanceComponent::default(),
        }
    }
}
