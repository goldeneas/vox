use std::{ops::Range, sync::Arc};

use bytemuck::{Pod, Zeroable};

use crate::{assets::{asset::Asset, asset_server::AssetServer}, components::{model::ModelComponent, multiple_instance::MultipleInstanceComponent, single_instance::SingleInstanceComponent}, Texture};

use super::{material::{Material, MaterialDescriptor}, mesh::{Mesh, MeshDescriptor}};

pub struct Model {
    meshes: Box<[Mesh]>,
    materials: Box<[Material]>,
    name: String,
}

impl Asset for Model {
    fn file_name(&self) -> &str {
        &self.name
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
    pub normal: [f32; 3],
}

// TODO: maybe this rc is not needed in the trait
pub trait IntoModel {
    fn to_model(&self, device: &wgpu::Device) -> Arc<Model>;
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
    fn draw_entity(&mut self,
        model_cmpnt: &'b ModelComponent,
        instance_cmpnt: &'b SingleInstanceComponent,
        camera_bind_group: &'b wgpu::BindGroup);
    fn draw_entity_multiple(&mut self,
        model_cmpnt: &'b ModelComponent,
        instance_cmpnt: &'b MultipleInstanceComponent,
        camera_bind_group: &'b wgpu::BindGroup);
}

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
        self.set_vertex_buffer(0, mesh.vertex_buffer().slice(..));
        self.set_index_buffer(mesh.index_buffer().slice(..), wgpu::IndexFormat::Uint32);
        self.set_bind_group(0, &material.bind_group(), &[]);
        self.set_bind_group(1, camera_bind_group, &[]);
        self.draw_indexed(0..mesh.num_indices(), 0, instances);
    }

    fn draw_model(&mut self,
        model: &'b Model,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material = &model.materials[mesh.material_id()];
            self.draw_mesh(mesh, material, camera_bind_group);
        }
    }

    fn draw_model_instanced(&mut self,
        model: &'b Model,
        instances: Range<u32>,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        for mesh in model.meshes.as_ref() {
            let material = &model.materials[mesh.material_id()];
            self.draw_mesh_instanced(mesh, material, instances.clone(), camera_bind_group);
        }
    }

    fn draw_entity(&mut self,
        model_cmpnt: &'b ModelComponent,
        instance_cmpnt: &'b SingleInstanceComponent,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.set_vertex_buffer(1, instance_cmpnt
            .instance_buffer()
            .slice(..));

        self.draw_model(&model_cmpnt.model,
            camera_bind_group
        );
    }

    fn draw_entity_multiple(&mut self,
        model_cmpnt: &'b ModelComponent,
        instance_cmpnt: &'b MultipleInstanceComponent,
        camera_bind_group: &'b wgpu::BindGroup
    ) {
        self.set_vertex_buffer(1, instance_cmpnt
            .instance_buffer()
            .slice(..));

        self.draw_model(&model_cmpnt.model,
            camera_bind_group
        );
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

// TODO: Make materials cached so that when reusing the same we dont create another
impl Model {
    pub fn new(device: &wgpu::Device,
        vertices: Box<[Vertex]>,
        indices: Box<[u32]>,
        diffuse_texture: Arc<Texture>,
        name: &str
    ) -> Self {
        let material = Material::new(device, MaterialDescriptor {
            name: format!("Material - {}", name),
            diffuse_texture,
        });

        let materials = Box::new([material]);

        let mesh = Mesh::new(device, MeshDescriptor {
            name: format!("Mesh - {}", name),
            vertices,
            indices
        });

        let meshes = Box::new([mesh]);

        let name = name.to_string();

        Model {
            materials,
            meshes,
            name,
        }
    }

    pub fn load(file_name: &str, asset_server: &mut AssetServer, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<Model> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let materials: Box<[Material]> = match materials_opt {
            Ok(tobj_materials) => {
                let materials = tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap();

                        Material::new(device, MaterialDescriptor {
                            name: format!("Material - {}", diffuse_texture_name),
                            diffuse_texture
                        })
                    }).collect::<Vec<_>>();

                materials.try_into().unwrap()
            },
            Err(_) => {
                let diffuse_texture = asset_server
                    .get_or_load("debug.png", device, queue)
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

        let name = file_name.to_string();

        let model = Model {
            meshes,
            materials,
            name,
        };

        Ok(model)
    }
}
