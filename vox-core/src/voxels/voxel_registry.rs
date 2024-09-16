use std::collections::HashMap;

use log::debug;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
pub enum VoxelType {
    AIR,
    DIRT,
}

pub type VoxelTypeIdentifier = u16;
pub struct VoxelRegistry {
    type_registry: HashMap<VoxelType, VoxelTypeIdentifier>,
}

impl Default for VoxelRegistry {
    fn default() -> Self {
        let type_registry = HashMap::new();
        let mut registry = Self {
            type_registry,
        };

        registry.register_type(VoxelType::AIR, 0);
        registry.register_type(VoxelType::DIRT, 1);

        registry
    }
}

impl VoxelRegistry {
    pub fn register_type(&mut self,
        voxel_type: VoxelType,
        id: VoxelTypeIdentifier
    ) {
        let opt = self.type_registry.insert(voxel_type, id);
        if opt.is_some() {
            debug!("Replaced voxel type {:?} with id {}", voxel_type, id);
        }
    }

    pub fn get_id(&self, voxel_type: VoxelType) -> Option<VoxelTypeIdentifier> {
        self.type_registry
            .get(&voxel_type)
            .copied()
    }

    pub fn get_type(&self, id: VoxelTypeIdentifier) -> Option<VoxelType> {
        self.type_registry
            .keys()
            .find_map(|voxel_type| {
                let candidate_id = self.get_id(*voxel_type)
                    .unwrap();

                if candidate_id == id {
                    Some(*voxel_type)
                } else {
                    None
                }
            })
    }
}
