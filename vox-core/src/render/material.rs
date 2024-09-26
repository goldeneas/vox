use core::panic;
use std::sync::Arc;

use crate::{resources::render_server::MaterialId, Texture};

#[derive(Debug)]
pub struct Material {
    diffuse_texture: Arc<Texture>,
    bind_group: wgpu::BindGroup,
    material_id: MaterialId,
}

// TODO: cache this
impl Material {
    pub fn new(diffuse_texture: Arc<Texture>,
        material_id: MaterialId,
        device: &wgpu::Device,
    ) -> Self {
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
            label: Some("Material Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(diffuse_texture.view()),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(diffuse_texture.sampler()),
                },
            ]
        });

        Material {
            diffuse_texture,
            bind_group,
            material_id,
        }
    }

    pub fn material_id(&self) -> MaterialId {
        self.material_id
    }

    pub fn diffuse_texture(&self) -> Arc<Texture> {
        self.diffuse_texture.clone()
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}

