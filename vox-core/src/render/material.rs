use std::rc::Rc;

use crate::Texture;

pub struct Material {
    diffuse_texture: Rc<Texture>,
    bind_group: wgpu::BindGroup,
}

pub struct MaterialDescriptor {
    pub name: String,
    pub diffuse_texture: Rc<Texture>,
}

impl Material {
    pub fn new(device: &wgpu::Device, descriptor: MaterialDescriptor) -> Self {
        let name = descriptor.name;
        let diffuse_texture = descriptor.diffuse_texture;

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                },
                wgpu::BindGroupLayoutEntry {
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                },
            ]
        });
        
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Material Bind Group - {}", name)),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ]
        });

        Material {
            diffuse_texture,
            bind_group,
        }
    }

    pub fn diffuse_texture(&self) -> &Texture {
        self.diffuse_texture.as_ref()
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

