use std::{ops::Range, sync::Arc};

use bevy_ecs::component::Component;

use crate::{asset::Asset, resources::asset_server::AssetServer, InstanceData, Texture};

use super::{material::Material, mesh::Mesh, render_server::{MaterialId, RenderServer}, vertex::Vertex};

#[derive(Debug, Component)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
    pub name: String,
}

impl Asset for Model {
    fn file_name(&self) -> &str {
        &self.name
    }
}

// TODO: Maybe make a way to cache these models too?
pub trait AsModel {
    fn to_model(&self, materials: Vec<Material>, device: &wgpu::Device) -> Model;
}

impl Model {
    pub fn load(file_name: &str,
        asset_server: &mut AssetServer,
        render_server: &mut RenderServer,
        device: &wgpu::Device,
        queue: &wgpu::Queue
    ) -> anyhow::Result<Model> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let material_ids: Vec<MaterialId> = match materials_opt {
            Ok(tobj_materials) => {
                tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap();

                        render_server.push_material(diffuse_texture, device)
                    }).collect::<Vec<_>>()
            },
            Err(_) => {
                let diffuse_texture = Texture::debug(asset_server, device, queue);
                let material_id = render_server
                    .push_material(diffuse_texture, device);

                vec![material_id]
            }
        };

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

                        Vertex {
                            position: [
                                m.mesh.positions[i * 3],
                                m.mesh.positions[i * 3 + 1],
                                m.mesh.positions[i * 3 + 2],
                            ],
                            tex_coords: [m.mesh.texcoords[i * 2], 1.0 - m.mesh.texcoords[i * 2 + 1]],
                            normal: normals,
                        }
                    }).collect::<Vec<_>>();

                let indices = m.mesh.indices.iter_mut()
                    .map(|index| *index as usize)
                    .collect::<Vec<_>>();

                // TODO: make this position customizable
                let instance_data = InstanceData::from_position((0.0, 0.0, 0.0));
                let instances = vec![instance_data];
                // we need to convert the meshes' material idxs to our own
                // material ids
                let material_id = material_ids[m.mesh.material_id.unwrap_or(0)];
                let mesh_id = render_server.free_mesh_id();

                let mesh = Mesh::new(&vertices,
                    &indices,
                    &instances,
                    material_id,
                    mesh_id,
                    device
                );
            }).collect::<Vec<_>>();

        let name = file_name.to_string();

        let model = Model {
            meshes,
            materials,
            name,
        };

        Ok(model)
    }
}
