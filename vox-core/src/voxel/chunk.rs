use cgmath::Vector3;
use rand::{distributions::Standard, prelude::Distribution, Rng};

use super::types::{ChunkCol, ChunkHeight, ChunkRow, CoordinateBound, VoxelBitmap};

// THE PLAN:
// we want to store a 32x32x32 chunk of voxels.
// for simplicity lets imagine that each voxel is of the same type
// we can store that in a bidimensional array of 32x32 u32
// where a sigle u32 repesents 32 voxels which
// can either be opaque or transparent.
// the binary rappresentation of this u32 will tell us
// which voxels are opaque (1s) and which are transparent (0s).
//
// we should store a row of voxels in an u32 (where row refers to the x axis);
// this way we are most likely to have a cache hit for the greedy mesh implementation
// if we move right/left on the u32.
//
// this means that the actual 32x32 bidimensional array is on the z and y axis
// while on the x axis we only have u32s
//
// EXAMPLE:
//
//     -Z
//      ^
//u32   |011001000...
//u32   |011100010...
//u32   |000000000...
//u32   |111011000...
//u32   |101011000...
//u32   |000011000...
//u32   *--------> +X

pub const CHUNK_ROWS: usize = 32;
pub const CHUNK_COLS: usize = 32;

// u64 column of binary represented voxels
// 64 x 64 => 64 x 64 x 64 total voxels represented by 1 bit
pub struct Chunk {
    pub voxels: [[VoxelBitmap ; CHUNK_ROWS] ; CHUNK_COLS],
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            voxels: rand::random(),
        }
    }

    pub fn generate_quads(&self, y: ChunkHeight) {
        let voxel_bitmaps = self.get_voxel_bitmap_at_height(y);

        let col = ChunkCol::parse(0).unwrap();

        loop {
            let voxel_bitmap = &voxel_bitmaps[col.0];
            println!("{}", voxel_bitmap);
        }
    }

    fn get_voxel_bitmap_at_row(&self, row: ChunkRow
    ) -> &[VoxelBitmap; CHUNK_ROWS] {
        &self.voxels[row.0]
    }

    fn get_voxel_bitmap_at_height(&self, y: ChunkHeight
    ) -> &[VoxelBitmap; CHUNK_ROWS] {
        let row = ChunkRow::from(y);
        &self.voxels[row.0]
    }

}

impl Distribution<VoxelBitmap> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> VoxelBitmap {
        let rand: u32 = rng.gen();
        VoxelBitmap(rand)
    }
}
