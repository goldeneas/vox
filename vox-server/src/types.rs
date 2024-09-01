//           +Y
//            |
//            |
//            |
//            |
//            |
//            |
//            *--------------- +X
//           /
//          /
//         /
//        /
//       +Z
//

use core::panic;
use std::fmt::{Debug, Display};

use block_mesh::{MergeVoxel, Voxel, VoxelVisibility};

use super::chunk::{CHUNK_COLS, CHUNK_ROWS};

pub trait CoordinateBound {
    fn parse(value: usize) -> Result<Self, ()> where Self: Sized;
}

// This represents the axes of a chunk.
// Look from the origin towards +X.
// ChunkRow -> Y AXIS
// 0th row: topmost row
// CHUNK_COLS-1th row: lowest row
//
// The opposite of this type is ChunkHeight
// which is more convenient to accept user input
pub struct ChunkRow(pub usize);
impl CoordinateBound for ChunkRow {
    fn parse(value: usize) -> Result<Self, ()> {
        if value >= CHUNK_ROWS {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

impl From<ChunkHeight> for ChunkRow {
    fn from(value: ChunkHeight) -> Self {
        let row = CHUNK_ROWS - value.0 - 1;
        ChunkRow::parse(row).unwrap()
    }
}

// ChunkCol -> Z AXIS
pub struct ChunkCol(pub usize);
impl CoordinateBound for ChunkCol {
    fn parse(value: usize) -> Result<Self, ()> {
        if value >= CHUNK_COLS {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

// represents a sequence of data for a single voxel
// from MSB to LSB
// 4 bits -> Voxel Type
// 4 bits -> Free

const VOXEL_TYPE_BITS: u8 = 4;

#[derive(Clone)]
pub struct VoxelData(pub u8);


impl VoxelData {
    pub fn get_type(&self) -> VoxelType {
        VoxelType::from(self)
    }
}

impl Voxel for VoxelData {
    fn get_visibility(&self) -> VoxelVisibility {
        if self.get_type() == VoxelType::AIR {
            return VoxelVisibility::Empty
        }

        VoxelVisibility::Opaque
    }
}

impl MergeVoxel for VoxelData {
    type MergeValue = VoxelType;

    fn merge_value(&self) -> Self::MergeValue {
        self.get_type()
    }
}

#[derive(PartialEq, Eq)]
pub enum VoxelType {
    AIR,
    DIRT,
}

impl VoxelType {
    pub fn from(data: &VoxelData) -> VoxelType {
        let type_id = data.0 >> (data.0);
        match type_id {
            0 => VoxelType::AIR,
            1 => VoxelType::DIRT,
            _ => panic!("Found unknown voxel type"),
        }
    }
}

// represents the y coordinate in the chunk
// 0: lowest coordinate
// CHUNK_ROWS - 1: highest coordinate
pub struct ChunkHeight(pub usize);
impl CoordinateBound for ChunkHeight {
    fn parse(value: usize) -> Result<Self, ()> {
        if value >= CHUNK_ROWS {
            Err(())
        } else {
            Ok(Self(value))
        }
    }
}

pub struct RelativeVoxelPosition(pub u32);
