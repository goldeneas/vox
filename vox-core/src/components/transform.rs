use bevy_ecs::component::Component;
use wgpu::util::DeviceExt;

use crate::Transform;

#[derive(Component)]
pub struct TransformComponent {
    pub transforms: [Transform],
}

impl TransformComponent {
    pub fn compute_buffer(&self, device: &wgpu::Device) -> wgpu::Buffer {
        let instances_raw = self.transforms.iter()
            .map(Transform::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instances_raw),
        });

        instance_buffer
    }
}
