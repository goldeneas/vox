use cgmath::Vector3;

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

// index referring to the position of a voxel in a column
// 0 => highest voxel
// ... => lower voxel
pub type VoxelIndex = u32;
pub type VoxelNumber = u32;
type VoxelRow = u32;

const MAX_HEIGHT: u32 = VoxelRow::BITS - 1;
const CHUNK_ROWS: usize = 32;
const CHUNK_COLS: usize = 32;

// u64 column of binary represented voxels
// 64 x 64 => 64 x 64 x 64 total voxels represented by 1 bit
pub struct Chunk {
    pub voxels: [[VoxelRow ; CHUNK_ROWS] ; CHUNK_COLS],
}

impl Chunk {
    pub fn new() -> Self {
        let s = "Could not find voxel column height correctly";
        debug_assert!(Self::find_height(0b0) == None, "{s}");
        debug_assert!(Self::find_height(0b1) == Some(MAX_HEIGHT), "{s}");
        debug_assert!(Self::find_height(0b010) == Some(1), "{s}");
        debug_assert!(Self::find_height(0b11010) == Some(1), "{s}");

        let voxels = [[0b10; 32] ; 32];

        Self {
            voxels,
        }
    }

    fn generate_meshes(&self) -> Option<Vec<(Vector3<u32>, Vector3<u32>)>> {
        let mut row = 0;
        let mut col = 0;
        let mut greedy_rows = 0;

        loop {
            if row == CHUNK_ROWS || col == CHUNK_COLS {
                println!("Reached the end of the chunk");
                return None;
            }

            // TODO on the second iter we dont need to retake the first row
            // but its most likely cached and its not expensive soo...
            let first = self.voxels[row][col];
            let second = self.voxels[row][col+1]; // first row but second column

            let c = first & second;

            if c == 0 {
                println!("greedy_rows: {}", greedy_rows);
                return None;
            }

            greedy_rows += 1;
            row += 1;
            col += 1;
        }

    }

    // returns index of first opaque voxel
    // take a look at this file for more info
    pub fn column_height(&self, column_id: (usize, usize)) -> Option<VoxelIndex> {
        let column = self.voxels[column_id.0][column_id.1];
        Self::find_height(column)
    }

    // for input:
    // MSB = bot voxel
    // LSB = top voxel
    fn find_height(mut column_number: VoxelNumber) -> Option<VoxelIndex> {
        if column_number == 0 { return None; }
        if column_number == 1 { return Some(MAX_HEIGHT); }

        let mut height = 0;
        column_number = column_number.reverse_bits();

        loop {
            if (column_number >> (MAX_HEIGHT-height)) == 1 {
                return Some(height);
            } else {
                height += 1;
            }
        }
    }
}
