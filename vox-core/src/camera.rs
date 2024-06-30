use bytemuck::{Pod, Zeroable};
use cgmath::{InnerSpace, SquareMatrix};
use winit::{event::{KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};
use bevy_ecs::prelude::*;

use crate::resources::input::InputRes;

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct CameraTransform {
    pub position: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

pub struct CameraController {
    pub speed: f32,
}

pub struct Camera {
    pub uniform: CameraUniform,
    transform: CameraTransform,
    controller: CameraController,
}

impl Camera {
    pub fn new(transform: CameraTransform, controller: CameraController) -> Self {
        let uniform = CameraUniform::new();

        Self {
            transform,
            controller,
            uniform,
        }
    }

    pub fn update(&mut self, world: &World) {
        self.controller.update(&mut self.transform, world);
        self.uniform.build_view_proj(&self.transform);
    }
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: cgmath::Matrix4::identity().into(),
        }
    }

    pub fn build_view_proj(&mut self, camera_transform: &CameraTransform) {
        let view = cgmath::Matrix4::look_at_rh(
            camera_transform.position,
            camera_transform.target,
            camera_transform.up
        );

        let proj = cgmath::perspective(
            cgmath::Deg(camera_transform.fovy),
            camera_transform.aspect,
            camera_transform.znear,
            camera_transform.zfar
        );

        self.view_proj = (OPENGL_TO_WGPU_MATRIX * proj * view).into();
    }
}

impl CameraController {
    pub fn update(&self, camera_transform: &mut CameraTransform, world: &World) {
        let forward = camera_transform.target - camera_transform.position;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        let input_res = world.get_resource::<InputRes>()
            .unwrap();

        if input_res.forward.is_pressed && forward_mag > self.speed {
            camera_transform.position += forward_norm * self.speed;
        }

        if input_res.backward.is_pressed {
            camera_transform.position -= forward_norm * self.speed;
        }

        let up_norm = camera_transform.up.normalize();
        let right_norm = forward_norm.cross(up_norm);

        if input_res.right.is_pressed {
            camera_transform.position += right_norm * self.speed; 
        }

        if input_res.left.is_pressed {
            camera_transform.position -= right_norm * self.speed;
        }
    }
}
