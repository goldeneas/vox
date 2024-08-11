use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::InstanceData;

use super::multiple_instance::MultipleInstanceComponent;

#[derive(Component)]
pub struct SingleInstanceComponent {
    instance_buffer: wgpu::Buffer,
}

impl SingleInstanceComponent {
    pub fn new(instance: &InstanceData, device: &wgpu::Device) -> Self {
        let instance_raw = instance.to_raw();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&[instance_raw]),
        });

        Self {
            instance_buffer,
        }
    }

    pub fn set_instance(&mut self, instance: &InstanceData, device: &wgpu::Device) {
        let instance_raw = instance.to_raw();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&[instance_raw]),
        });

        self.instance_buffer = instance_buffer;
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }
}
