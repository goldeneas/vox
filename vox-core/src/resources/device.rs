use std::sync::Arc;

use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct DeviceRes {
    pub device: Arc<wgpu::Device>
}
