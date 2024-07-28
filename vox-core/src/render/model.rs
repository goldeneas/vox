use std::ops::Range;

use bytemuck::{Pod, Zeroable};
use wgpu::util::{DeviceExt, RenderEncoder};

use crate::{ Instance, InstanceRaw, Texture, Vertex };

pub struct Model {
    pub meshes: Vec<ModelMesh>,
    pub materials: Vec<Material>,
}

pub struct Material {
    pub name: String,
    pub diffuse_texture: Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct ModelMesh {
    pub name: String,
    pub index_buffer: wgpu::Buffer,
    pub vertex_buffer: wgpu::Buffer,
    pub num_indices: u32,
    pub material_id: usize,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct ModelVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

pub trait DrawModel<'b> {
    fn draw_mesh(&mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_mesh_instanced(&mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_model(&mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_model_instanced(&mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_model_as(&mut self,
        model: &'b Model,
        instance_buffer: &'b wgpu::Buffer,
        camera_bind_group: &'b wgpu::BindGroup);
}

impl<'a, 'b> DrawModel<'b> for wgpu::RenderPass<'a>
where 'b: 'a {
    fn draw_mesh(&mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    fn draw_mesh_instanced(&mut self,
        mesh: &'b ModelMesh,
        material: &'b Material,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
        self.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group, &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices, 0, instances);
    }

    fn draw_model(&mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material_id];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }

    fn draw_model_instanced(&mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        for mesh in &model.meshes {
            let material = &model.materials[mesh.material_id];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }

    fn draw_model_as(&mut self,
        model: &'b Model,
        instance_buffer: &'b wgpu::Buffer,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.set_vertex_buffer(1, instance_buffer.slice(..));
        self.draw_model(model, camera_bind_group);
    }
}

impl Vertex for ModelVertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            array_stride: std::mem::size_of::<ModelVertex>() as wgpu::BufferAddress,
            attributes: &[
                wgpu::VertexAttribute {
                    shader_location: 0,
                    offset: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    shader_location: 1,
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    shader_location: 2,
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    format: wgpu::VertexFormat::Float32x3,
                },
            ]
        }
    }
}

// TODO: we dont want to be constrained on also having  a materials file. We should make the
// materials optional
impl Model {
    pub fn new(device: &wgpu::Device,
        vertices: &[ModelVertex],
        indices: &[u32],
        diffuse_texture: Texture,
        name_opt: Option<&str>
    ) -> Self {
        let model_name = name_opt.unwrap_or_default();

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
            label: Some("Model Bind Group"),
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

        let mut materials = Vec::new();
        let material = Material {
            name: format!("Model Material - {}", model_name),
            diffuse_texture,
            bind_group,
        };

        materials.push(material);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Primitive Vertex Buffer"),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(vertices),
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Primitive Index Buffer"),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(indices),
        });

        let num_indices = indices.len() as u32;

        let mut meshes = Vec::new();

        let mesh = ModelMesh {
            name: format!("Model Mesh - {}", model_name),
            index_buffer,
            num_indices,
            vertex_buffer,
            material_id: 0,
        };

        meshes.push(mesh);

        Model {
            materials,
            meshes,
        }
    }

    pub fn load(file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<Model> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let materials = materials_opt?
            .into_iter()
            .map(|m| {
                let diffuse_texture_name = &m.diffuse_texture.unwrap();
                let diffuse_texture = Texture::load(diffuse_texture_name, device, queue)
                    .unwrap();

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
                    label: Some(diffuse_texture_name),
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
                    name: m.name,
                    diffuse_texture,
                    bind_group
                }
            }).collect::<Vec<_>>();

        let meshes = models.into_iter()
            .map(|m| {
                let vertices = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        let mut normals = [0.0, 0.0, 0.0];
                        if !m.mesh.normals.is_empty() { 
                            normals = [
                                m.mesh.normals[i * 2],
                                m.mesh.normals[i * 2 + 1],
                                m.mesh.normals[i * 2 + 2],
                            ];
                        }

                        ModelVertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: normals,
                        }
                    }).collect::<Vec<_>>();

                let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Vertex Buffer", file_name)),
                    usage: wgpu::BufferUsages::VERTEX,
                    contents: bytemuck::cast_slice(&vertices),
                });

                let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{:?} Index Buffer", file_name)),
                    usage: wgpu::BufferUsages::INDEX,
                    contents: bytemuck::cast_slice(&m.mesh.indices),
                });

                ModelMesh {
                    name: file_name.to_string(),
                    index_buffer,
                    vertex_buffer,
                    material_id: m.mesh.material_id.unwrap_or(0),
                    num_indices: m.mesh.indices.len() as u32,
                }
            }).collect::<Vec<_>>();

        Ok(Model {
            meshes,
            materials,
        })
    }
}
