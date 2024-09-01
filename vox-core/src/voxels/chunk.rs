use std::usize;

use block_mesh::{greedy_quads, ndshape::{ConstShape, ConstShape3u32}, GreedyQuadsBuffer, RIGHT_HANDED_Y_UP_CONFIG};

use super::voxel::VoxelData;

type ChunkShape = ConstShape3u32<18, 18, 18>;

pub struct Chunk {
    voxels: [VoxelData; ChunkShape::SIZE as usize],
    buffer: GreedyQuadsBuffer,
}

impl Chunk {
    pub fn new() -> Self {
        let mut voxels = [VoxelData::from(0b0); ChunkShape::SIZE as usize];
        for i in 0..ChunkShape::SIZE {
            let [x, y, z] = ChunkShape::delinearize(i);
            voxels[i as usize] = if ((x * x + y * y + z * z) as f32).sqrt() < 15.0 {
                VoxelData::from(0b11111111)
            } else {
                VoxelData::from(0b0)
            };
        }

        let buffer = GreedyQuadsBuffer::new(voxels.len());

        Self {
            voxels,
            buffer,
        }
    }

    pub fn greedy_mesh(&mut self) -> &GreedyQuadsBuffer {
        greedy_quads(&self.voxels,
            &ChunkShape {},
            [0; 3],
            [17 ; 3],
            &RIGHT_HANDED_Y_UP_CONFIG.faces,
            &mut self.buffer
        );

        &self.buffer
    }
}
