use bevy_ecs::prelude::*;
use cgmath::{Matrix4, Quaternion, SquareMatrix, Zero};

use crate::{components::{camerable::CamerableComponent, speed::SpeedComponent, transform::TransformComponent}, Transform};

// TODO: dont really like how cameras are structured
#[derive(Bundle)]
pub struct CameraBundle {
    speed: SpeedComponent,
    transform: TransformComponent,
    camerable: CamerableComponent,
}

impl CameraBundle {
    pub fn debug(config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            position: TransformComponent {
                transforms: [Transform {
                    position: (0.1, 0.2, 0.3).into(),
                    rotation: Quaternion::zero(),
                }]
            },
            speed: SpeedComponent {
                speed: 0.3,
            },
            camerable: CamerableComponent {
                target: (0.0, 0.0, 0.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: config.width as f32 / config.height as f32,
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
