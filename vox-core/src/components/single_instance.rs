use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::InstanceData;

#[derive(Component, Default)]
pub struct SingleInstanceComponent {
    instance_buffer: Option<wgpu::Buffer>,
}

impl SingleInstanceComponent {
    pub fn set_instance(&mut self, instance: &InstanceData, device: &wgpu::Device) {
        let instance_raw = instance.to_raw();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&[instance_raw]),
        });

        self.instance_buffer = Some(instance_buffer);
    }

    pub fn instance_buffer(&self) -> Option<&wgpu::Buffer> {
        self.instance_buffer
            .as_ref()
    }
}
