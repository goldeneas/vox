use std::sync::Arc;

use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::{InstanceData, Model};

#[derive(Component)]
pub struct ObjectComponent {
    model: Arc<Model>,
    num_instances: usize,
    instance_buffer: wgpu::Buffer,
}

impl ObjectComponent {
    pub fn new(
        model: Arc<Model>,
        instances: &[InstanceData],
        device: &wgpu::Device,
    ) -> Self {
        let instances_raw = instances
            .iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        let num_instances = instances.len();

        Self {
            model,
            num_instances,
            instance_buffer,
        }
    }

    pub fn model(&self) -> &Arc<Model> {
        &self.model
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn num_instances(&self) -> usize {
        self.num_instances
    }
}
