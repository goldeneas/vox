use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::InstanceData;

#[derive(Component)]
pub struct MultipleInstanceComponent {
    num_instances: u32,
    instance_buffer: wgpu::Buffer,
}

impl MultipleInstanceComponent {
    pub fn new(instances: &[InstanceData], device: &wgpu::Device) -> Self {
        let instances_raw = instances
            .iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        let num_instances = instances.len() as u32;

        Self {
            instance_buffer,
            num_instances,
        }
    }

    pub fn set_instances(&mut self, instances: &[InstanceData], device: &wgpu::Device) {
        let instances_raw = instances
            .iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        self.num_instances = instances.len() as u32;
        self.instance_buffer = instance_buffer;
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn num_instances(&self) -> u32 {
        self.num_instances
    }
}
