use std::sync::Arc;

use bevy_ecs::prelude::*;
use cgmath::{Quaternion, Zero};

use crate::{components::{model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent}, AsModel, Model};

#[derive(Bundle)]
pub struct GameObject {
    position: PositionComponent,
    model: ModelComponent,
    instance: SingleInstanceComponent,
    rotation: RotationComponent,
}

impl GameObject {
    pub fn new(into_model: impl AsModel,
        position: (f32, f32, f32),
        rotation: Quaternion<f32>,
        device: &wgpu::Device
    ) -> Self {
        let model = into_model.into_model(device);

        Self {
            position: PositionComponent {
                position: position.into() 
            },
            model: ModelComponent {
                model,
            },
            rotation: RotationComponent {
                quaternion: rotation,
            },
            instance: SingleInstanceComponent::default(),
        }
    }

    pub fn debug(model: impl AsModel, device: &wgpu::Device) -> Self {
        let model = model.into_model(device);

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
