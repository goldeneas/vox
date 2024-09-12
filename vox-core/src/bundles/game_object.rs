use std::sync::Arc;

use bevy_ecs::prelude::*;
use cgmath::{Quaternion, Zero};

use crate::{components::{model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent}, Model};

#[derive(Bundle)]
pub struct GameObject {
    pub position: PositionComponent,
    pub model: ModelComponent,
    pub instance: SingleInstanceComponent,
    pub rotation: RotationComponent,
}

impl GameObject {
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
