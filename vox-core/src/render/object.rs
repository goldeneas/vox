use wgpu::util::DeviceExt;

use crate::{Instance, IntoModel, Model};

pub struct Object {
    pub model: Model,
    pub instance_buffer: wgpu::Buffer,
}

impl Object {
    pub fn new(device: &wgpu::Device, model: impl IntoModel, instances: &[Instance]) -> Self {
        let instance_data = instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instance_data),
        });

        let base_model = model.to_model(device);

        Self {
            model: base_model,
            instance_buffer,
        }
    }
}
