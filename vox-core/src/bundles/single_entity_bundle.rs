use bevy_ecs::prelude::*;
use cgmath::{Matrix4, SquareMatrix};

use crate::components::{camerable::CamerableComponent, model::ModelComponent, position::PositionComponent, rotation::RotationComponent, single_instance::SingleInstanceComponent, speed::SpeedComponent};

#[derive(Bundle)]
pub struct SingleEntity {
    position: PositionComponent,
    model: ModelComponent,
    instance: SingleInstanceComponent,
    rotation: RotationComponent,
}

impl Default for SingleEntity {
    fn default() -> Self {
        Self {
            position: PositionComponent {
                position: (0.0, 1.0, 2.0).into() 
            },
            speed: SpeedComponent {
                speed: 1.0,
            },
            camerable: CamerableComponent {
                target: (0.0, 0.0, 0.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: 1920.0 / 1080.0,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
                yaw: 0.0,
                pitch: 0.0,
                last_mouse_pos: (0.0, 0.0),
                view_proj: Matrix4::identity().into(),
            }
        }
    }
}
