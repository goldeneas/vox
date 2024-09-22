use crate::InstanceData;

use super::{mesh::AsMesh, render_server::MaterialId, vertex::{Index, Vertex}};

// TODO rename this to mesh 
// and mesh to meshraw
#[derive(Debug)]
pub struct PhantomMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<Index>,
    pub instances: Vec<InstanceData>,
    pub material_id: MaterialId,
}

impl AsMesh for PhantomMesh {
    fn vertices(&self) -> Vec<Vertex> {
        self.vertices.clone()
    }

    fn indices(&self) -> Vec<Index> {
        self.indices.clone()
    }

    fn instances(&self) -> Vec<InstanceData> {
        self.instances.clone()
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }
}
