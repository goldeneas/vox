use bytemuck::{Pod, Zeroable};
use cgmath::{InnerSpace, SquareMatrix};
use winit::{event::{KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

#[rustfmt::skip]
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.5,
    0.0, 0.0, 0.0, 1.0,
);

pub struct CameraTransform {
    pub eye: cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up: cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy: f32,
    pub znear: f32,
    pub zfar: f32,
}

pub struct Camera {
    transform: CameraTransform,
}

impl Camera {
    pub fn new(camera_transform: CameraTransform) -> Self {
        Self {
            transform: camera_transform
        }
    }

    pub fn build_view_projection_matrix(&self) -> cgmath::Matrix4<f32> {
        let view = cgmath::Matrix4::look_at_rh(
            self.transform.eye,
            self.transform.target,
            self.transform.up
        );

        let proj = cgmath::perspective(
            cgmath::Deg(self.transform.fovy),
            self.transform.aspect,
            self.transform.znear,
            self.transform.zfar
        );

        return OPENGL_TO_WGPU_MATRIX * proj * view;
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    // 4x4 Matrix
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self { view_proj: cgmath::Matrix4::identity().into(), }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = camera.build_view_projection_matrix().into();
    }
}

pub struct CameraController {
    speed: f32,
    is_forward_pressed: bool,
    is_backward_pressed: bool,
    is_right_pressed: bool,
    is_left_pressed: bool,
}

impl CameraController {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            is_forward_pressed: false,
            is_backward_pressed: false,
            is_right_pressed: false,
            is_left_pressed: false,
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                event: KeyEvent {
                    state,
                    physical_key: PhysicalKey::Code(keycode), 
                    ..
                },
                ..
            } => {
                let is_pressed = state.is_pressed();

                match keycode {
                    KeyCode::KeyW => {
                        self.is_forward_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyA => {
                        self.is_left_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyS => {
                        self.is_backward_pressed = is_pressed;
                        true
                    },

                    KeyCode::KeyD => {
                        self.is_right_pressed = is_pressed;
                        true
                    },

                    _ => false
                }
            }

            _ => false
        }
    }

    pub fn update(&self, camera: &mut Camera) {
        let camera_transform = &mut camera.transform;

        let forward = camera_transform.target - camera_transform.eye;
        let forward_norm = forward.normalize();
        let forward_mag = forward.magnitude();

        if self.is_forward_pressed && forward_mag > self.speed {
            camera_transform.eye += forward_norm * self.speed;
        }

        if self.is_backward_pressed {
            camera_transform.eye -= forward_norm * self.speed;
        }

        let up_norm = camera_transform.up.normalize();
        let right_norm = forward_norm.cross(up_norm);

        if self.is_right_pressed {
            camera_transform.eye += right_norm * self.speed; 
        }

        if self.is_left_pressed {
            camera_transform.eye -= right_norm * self.speed;
        }
    }
}
