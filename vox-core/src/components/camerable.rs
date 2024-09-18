use bevy_ecs::prelude::*;
use cgmath::{Matrix4, Point3, SquareMatrix, Vector3};

pub type CameraUniform = [[f32;4];4];

#[derive(Component)]
pub struct CameraComponent {
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
    pub position: Point3<f32>,
    pub speed: f32,
}

impl CameraComponent {
    pub fn debug(config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            position: (0.1, 0.2, 0.3).into(),
            speed: 0.3,
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
