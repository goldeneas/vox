use std::{ops::Range, rc::Rc};

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::Texture;

use super::object::Object;

pub struct Model {
    meshes: Box<[Mesh]>,
    materials: Box<[Material]>,
}

pub struct Material {
    diffuse_texture: Rc<Texture>,
    bind_group: wgpu::BindGroup,
}

pub struct MaterialDescriptor {
    pub name: String,
    pub diffuse_texture: Rc<Texture>,
}

pub struct Mesh {
    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    num_indices: u32,
    // the material assigned to this mesh from the materials
    material_id: usize, 
}

pub struct MeshDescriptor {
    pub name: String,
    pub indices: Box<[u32]>,
    pub vertices: Box<[Vertex]>,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

// TODO: maybe move to convert.rs
pub trait IntoModel {
    fn to_model(&self, device: &wgpu::Device) -> Rc<Model>;
}

pub trait DrawObject<'b> {
    fn draw_mesh(&mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_mesh_instanced(&mut self,
        mesh: &'b Mesh,
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
    fn draw_object(&mut self,
        object: &'b Object,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_object_instanced(&mut self,
        object: &'b Object,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup);
}

// TODO: maybe let the non instanced versions of these function spawn all instances instead of just
// one
impl<'a, 'b> DrawObject<'b> for wgpu::RenderPass<'a>
where 'b: 'a {
    fn draw_mesh(&mut self,
        mesh: &'b Mesh,
        material: &'b Material,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.draw_mesh_instanced(mesh, material, 0..1, camera_bind_group);
    }

    // TODO: set the instance buffer to empty before drawing
    // since it might have been set before when calling an instanced version
    // of the drawing
    fn draw_mesh_instanced(&mut self,
        mesh: &'b Mesh,
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
        for mesh in model.meshes.as_ref() {
            let material = &model.materials[mesh.material_id];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }

    fn draw_model_instanced(&mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material = &model.materials[mesh.material_id];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }

    fn draw_object(&mut self,
        object: &'b Object,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.draw_object_instanced(object, 0..1, camera_bind_group);
    }

    fn draw_object_instanced(&mut self,
        object: &'b Object,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        // FIXME: This slot is hardcoded since we only have one shader for now
        self.set_vertex_buffer(1, object.instance_buffer.slice(..));
        self.draw_model_instanced(&object.model, instances, camera_bind_group);
    }
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            step_mode: wgpu::VertexStepMode::Vertex,
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
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

impl Material {
    fn new(device: &wgpu::Device, descriptor: MaterialDescriptor) -> Self {
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
}

impl Mesh {
    pub fn new(device: &wgpu::Device, descriptor: MeshDescriptor) -> Self {
        let name = descriptor.name;
        let indices = descriptor.indices;
        let vertices = descriptor.vertices;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Vertex Buffer", name)),
            usage: wgpu::BufferUsages::VERTEX,
            contents: bytemuck::cast_slice(&vertices),
        });
        
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{:?} Index Buffer", name)),
            usage: wgpu::BufferUsages::INDEX,
            contents: bytemuck::cast_slice(&indices),
        });

        let num_indices = indices.len() as u32;

        let material_id = 0;
        
        Mesh {
            index_buffer,
            vertex_buffer,
            material_id,
            num_indices,
        }
    }
}

// TODO: we dont want to be constrained on also having  a materials file. We should make the
// materials optional
// TODO: Make materials cached so that when reusing the same we dont create another
impl Model {
    pub fn new(device: &wgpu::Device,
        vertices: Box<[Vertex]>,
        indices: Box<[u32]>,
        diffuse_texture: Rc<Texture>,
        name_opt: Option<&str>
    ) -> Rc<Self> {
        let model_name = name_opt.unwrap_or_default();

        let material = Material::new(device, MaterialDescriptor {
            name: format!("Material - {}", model_name),
            diffuse_texture,
        });

        let materials = Box::new([material]);

        let mesh = Mesh::new(device, MeshDescriptor {
            name: format!("Mesh - {}", model_name),
            vertices,
            indices
        });

        let meshes = Box::new([mesh]);

        let model = Model {
            materials,
            meshes,
        };

        Rc::new(model)
    }

    pub fn load(file_name: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<Rc<Model>> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let materials: Box<[Material]> = match materials_opt {
            Ok(tobj_materials) => {
                let materials = tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = Texture::load(diffuse_texture_name, device, queue)
                            .unwrap();

                        Material::new(device, MaterialDescriptor {
                            name: format!("Material - {}", diffuse_texture_name),
                            diffuse_texture
                        })
                    }).collect::<Vec<_>>();

                materials.try_into().unwrap()
            },
            Err(_) => {
                let diffuse_texture = Texture::load("debug.png", device, queue)
                    .unwrap();

                let material = Material::new(device, MaterialDescriptor {
                    name: "Debug Material".to_owned(),
                    diffuse_texture
                });

                Box::new([material])
            }
        };

        let meshes: Box<[Mesh]> = models.into_iter()
            .map(|m| {
                let vertices: Box<[Vertex]> = (0..m.mesh.positions.len() / 3)
                    .map(|i| {
                        let mut normals = [0.0, 0.0, 0.0];
                        if !m.mesh.normals.is_empty() { 
                            normals = [
                                m.mesh.normals[i * 2],
                                m.mesh.normals[i * 2 + 1],
                                m.mesh.normals[i * 2 + 2],
                            ];
                        }

                        Vertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: normals,
                        }
                    }).collect::<Vec<_>>()
                .try_into().unwrap();

                let indices: Box<[u32]> = m.mesh.indices
                    .try_into().unwrap();

                Mesh::new(device, MeshDescriptor {
                    vertices,
                    indices,
                    name: file_name.to_owned(),
                })
            }).collect::<Vec<_>>()
        .try_into().unwrap();

        let model = Model {
            meshes,
            materials,
        };

        Ok(Rc::new(model))
    }
}
