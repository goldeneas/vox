use crate::AsModel;

use super::mesh::AsMesh;

pub struct PhantomModel {
    pub meshes: Vec<Box<dyn AsMesh>>,
}

impl AsModel for PhantomModel {
    fn meshes(&self) -> Vec<Box<dyn AsMesh>> {
        self.meshes()
    }
}
