use crate::{resources::render_server::MaterialId, InstanceData};

use super::{mesh::AsMesh, vertex::{Index, Vertex}};

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
    fn vertices(&self) -> &[Vertex] {
        &self.vertices
    }

    fn indices(&self) -> &[Index] {
        &self.indices
    }

    fn instances(&self) -> &[InstanceData] {
        &self.instances
    }

    fn material_id(&self) -> MaterialId {
        self.material_id
    }
}
