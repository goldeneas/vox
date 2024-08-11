use bevy_ecs::prelude::*;
use cgmath::{Point3, Vector3};

pub type CameraUniform = [[f32;4];4];

#[derive(Component)]
pub struct CamerableComponent {
    pub target: Point3<f32>,
    pub aspect: f32,
    pub znear: f32,
    pub zfar: f32,
    pub fovy: f32,
    pub last_mouse_pos: (f64, f64),
    pub up: Vector3<f32>,
    pub yaw: f32,
    pub pitch: f32,
    pub view_proj: CameraUniform,
}
