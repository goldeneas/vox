use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

type PackedDataType = u8;

const VOXEL_TYPE_BITS: u32 = 4;
const VOXEL_VISIBILITY_BITS: u32 = 1;

#[derive(PartialEq, Eq)]
pub enum VoxelType {
    AIR,
    DIRT,
}

// TODO: change this
impl From<&VoxelData> for VoxelType {
    fn from(value: &VoxelData) -> Self {
        let data = value.0;
        let type_identifier = data &
            (PackedDataType::MAX >> (PackedDataType::BITS - VOXEL_TYPE_BITS));

        match type_identifier {
            0 => Self::AIR,
            1 => Self::DIRT,
            _ => todo!("Unknown voxel type flag. Found {}", type_identifier),
        }
    }
}

// VOXEL DATA
// MSB 0b00000000 LSB
// FROM LSB TO MSB
// (RIGHT TO LEFT)

// TODO ADD DEBUG ASSERT

#[derive(Clone, Copy, Debug)]
pub struct VoxelData(PackedDataType);

impl From<PackedDataType> for VoxelData {
    fn from(value: PackedDataType) -> Self {
        VoxelData(value)
    }
}

impl Voxel for VoxelData {
    fn get_visibility(&self) -> VoxelVisibility {
        let data = self.0;
        let visibility_flag = data >> VOXEL_TYPE_BITS;
        let is_visible = visibility_flag & 1;

        if is_visible == 1 {
            VoxelVisibility::Opaque
        } else {
            VoxelVisibility::Empty
        }
    }
}

impl MergeVoxel for VoxelData {
    type MergeValue = VoxelType;

    fn merge_value(&self) -> Self::MergeValue {
        VoxelType::from(self)
    }
}
