use binary_greedy_meshing as bgm;

pub struct VoxelPosition((usize, usize, usize));

impl VoxelPosition {
    pub fn index(&self) -> usize {
        let position = self.0;
        bgm::pad_linearize(position.0, position.1, position.2)
    }
}

// TODO: setting a voxel at (62 62 62) doesnt actually work
impl From<(usize, usize, usize)> for VoxelPosition {
    fn from(value: (usize, usize, usize)) -> Self {
        let x = value.0;
        let y = value.1;
        let z = value.2;

        debug_assert!(x <= 62, "Tried changing a voxel out of bounds on x axis!");
        debug_assert!(y <= 62, "Tried changing a voxel out of bounds on y axis!");
        debug_assert!(z <= 62, "Tried changing a voxel out of bounds on z axis!");

        Self((x, y, z))
    }
}
