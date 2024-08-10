use bevy_ecs::prelude::*;

use crate::camera::Camera;

#[derive(Resource)]
pub struct CameraContext {
    pub camera_buffer: wgpu::Buffer,
}
