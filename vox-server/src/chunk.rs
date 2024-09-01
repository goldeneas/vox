use block_mesh::ndshape::ConstShape3u32;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use crate::types::{RelativeVoxelPosition, VoxelData};

use super::types::{ChunkCol, ChunkHeight, ChunkRow, CoordinateBound};

pub const CHUNK_ROWS: usize = 16;
pub const CHUNK_COLS: usize = 16;
pub const CHUNK_SLICES: usize = 16;

type ChunkShape = ConstShape3u32<18, 18, 18>;

pub struct Chunk {
    // looking at the bottom of the chunk from the top
    // X -> COL
    // Y -> ROW
    // Z -> SLICE
    pub voxels: [[[VoxelData ; CHUNK_SLICES] ; CHUNK_ROWS] ; CHUNK_COLS ],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: rand::random(),
        }
    }

    fn get_voxel_bitmaps_at_row(&self, row: ChunkRow
    ) -> &[VoxelData; CHUNK_COLS] {
        &self.voxels[row.0]
    }

    fn get_voxel_bitmaps_at_height(&self, y: ChunkHeight
    ) -> &[VoxelData; CHUNK_COLS] {
        let row = ChunkRow::from(y);
        &self.voxels[row.0]
    }

}

impl Distribution<VoxelData> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VoxelData {
        let rand: u8 = rng.gen();
        VoxelData(rand)
    }
}
