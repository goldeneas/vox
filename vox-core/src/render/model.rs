use std::{ops::Range, sync::Arc};

use crate::{asset::Asset, components::transform::TransformComponent, resources::asset_server::AssetServer, Texture};

use super::{material::Material, mesh::Mesh, vertex::Vertex};

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
    fn to_model(&self, materials: Vec<Material>) -> Model;
}

impl Model {
    pub fn load(file_name: &str, asset_server: &mut AssetServer, device: &wgpu::Device, queue: &wgpu::Queue) -> anyhow::Result<Model> {
        let (models, materials_opt) = tobj::load_obj(file_name, &tobj::GPU_LOAD_OPTIONS)
            .expect("Could not load file OBJ file");

        let materials: Vec<Material> = match materials_opt {
            Ok(tobj_materials) => {
                tobj_materials
                    .into_iter()
                    .map(|m| {
                        let diffuse_texture_name = &m.diffuse_texture.unwrap();
                        let diffuse_texture = asset_server
                            .get_or_load(diffuse_texture_name, device, queue)
                            .unwrap();

                        Material::new(device,
                            diffuse_texture,
                            &format!("Material - {}", diffuse_texture_name),
                        )
                    }).collect::<Vec<_>>()
            },
            Err(_) => {
                let diffuse_texture = Texture::debug(asset_server, device, queue);

                let material = Material::new(device,
                    diffuse_texture,
                    "Debug Material",
                );

                vec!(material)
            }
        };

        let meshes: Vec<Mesh> = models.into_iter()
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

                let name = format!("Mesh - {}", file_name);
                let indices = m.mesh.indices;
                let material_id = m.mesh.material_id;

                Mesh {
                    vertices,
                    indices,
                    material_id,
                    name,
                }
            }).collect::<Vec<_>>()
        .into();

        let name = file_name.to_string();

        let model = Model {
            meshes,
            materials,
            name,
        };

        Ok(model)
    }
}
