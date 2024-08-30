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

use std::fmt::{Debug, Display};

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

// represents a bitmap of voxels
// 1: opaque voxel
// 0: transparent voxel
pub struct VoxelBitmap(pub u32);

impl Display for VoxelBitmap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VoxelBitmap [{:#034b}]", self.0)
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
