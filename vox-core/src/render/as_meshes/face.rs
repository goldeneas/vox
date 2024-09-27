use crate::{render::{mesh::{AsMesh, MeshPosition}, face_orientation::FaceOrientation, vertex::{Index, Vertex}}, resources::render_server::MaterialId, InstanceData};

#[derive(Hash, PartialEq, Eq, Debug)]
pub struct FaceDescriptor {
    pub orientation: FaceOrientation,
    pub width: u32,
    pub height: u32,
    pub material_id: MaterialId,
}
