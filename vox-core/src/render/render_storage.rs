use super::{material::Material, mesh::Mesh};

#[derive(Clone, Copy, Debug)]
pub enum MaterialId {
    Index(usize),
}

pub struct RenderStorage {
    meshes: Vec<Mesh>,
    materials: Vec<Material>,
}

impl RenderStorage {
    pub fn new() -> Self {
        let meshes = Vec::new();
        let materials = Vec::new();

        Self {
            meshes,
            materials,
        }
    }

    pub fn push_material(&mut self, material: Material) -> MaterialId {
        self.materials.push(material);

        let idx = self.materials().len() - 1;
        MaterialId::Index(idx)
    }

    pub fn push_mesh(&mut self, mesh: Mesh) {
        self.meshes.push(mesh);
    }

    pub fn meshes(&self) -> &Vec<Mesh> {
        &self.meshes
    }

    pub fn materials(&self) -> &Vec<Material> {
        &self.materials
    }
}
