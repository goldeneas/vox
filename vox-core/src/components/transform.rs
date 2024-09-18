use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::InstanceData;

#[derive(Component)]
pub struct TransformComponent {
    pub instances_data: Vec<InstanceData>,
}

impl TransformComponent {
    pub fn compute_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let instances_raw = self.instances_data.iter()
            .map(InstanceData::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        instance_buffer
    }
}

impl From<InstanceData> for TransformComponent {
    fn from(value: InstanceData) -> Self {
        Self {
            instances_data: vec![value],
        }
    }
}
