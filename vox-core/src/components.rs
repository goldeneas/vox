use std::sync::Arc;

use bevy_ecs::prelude::*;
use wgpu::util::DeviceExt;

use crate::{InstanceTransform, Model};

#[derive(Component)]
pub struct PositionComponent {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


#[derive(Component)]
pub struct RotationComponent {
    pub quaternion: cgmath::Quaternion<f32>,
}

#[derive(Component)]
pub struct RenderComponent {
    model: Arc<Model>,
    num_instances: u32,
    instance_buffer: Option<wgpu::Buffer>,
}

impl RenderComponent {
    pub fn new(model: Arc<Model>) -> Self {
        let instance_buffer = None;

        let num_instances = 0;

        Self {
            model,
            instance_buffer,
            num_instances,
        }
    }

    pub fn set_instances(&mut self, instances: &[InstanceTransform], device: &wgpu::Device) {
        let instance_data = instances
            .iter()
            .map(InstanceTransform::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instance_data),
        });

        self.num_instances = instances.len() as u32;
        self.instance_buffer = Some(instance_buffer);
    }

    pub fn model(&self) -> &Model {
        self.model.as_ref()
    }

    pub fn instance_buffer(&self) -> Option<&wgpu::Buffer> {
        self.instance_buffer.as_ref()
    }

    pub fn num_instances(&self) -> u32 {
        self.num_instances
    }
}
