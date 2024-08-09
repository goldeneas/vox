use std::rc::Rc;

use wgpu::util::DeviceExt;

use crate::{Instance, Model};

pub struct Object {
    model: Rc<Model>,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
}

impl Object {
    pub fn new(device: &wgpu::Device, model: Rc<Model>, instances: &[Instance]) -> Self {
        let instance_data = instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instance_data),
        });

        let num_instances = instances.len() as u32;

        Self {
            model,
            instance_buffer,
            num_instances,
        }
    }

    //pub fn set_transform(&mut self, position: cgmath::Vector3<f32>, rotation: cgmath::Quaternion<f32>, device: &wgpu::Device) {
    //    assert!(self.num_instances == 1,
    //        "Tried setting a transform for an object with multiple instances! Did you mean to use set_instances?");

    //    self.set_instances(&[
    //        Instance {
    //            position,
    //            rotation,
    //        }
    //    ], device);
    //}

    pub fn set_instances(&mut self, instances: &[Instance], device: &wgpu::Device) {
        let instance_data = instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();

        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Object Instance Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&instance_data),
        });

        self.num_instances = instances.len() as u32;
        self.instance_buffer = instance_buffer;
    }

    pub fn model(&self) -> &Model {
        self.model.as_ref()
    }

    pub fn instance_buffer(&self) -> &wgpu::Buffer {
        &self.instance_buffer
    }

    pub fn num_instances(&self) -> u32 {
        self.num_instances
    }
}
