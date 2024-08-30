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
//
// implementing a greedy mesher now requires extremely fast bitwise operations.
// this could be the possible algorith implementation:
// 1. load next u32 (voxels[a][b])
// 1a. the first row should now be in cache -> subsequent calls should be faster
// 2. bitwise and the first and second u32
// 2a. if 0 there is no possibility of greedy meshing -> generate normal mesh -> goto begin
// 2b. if not 0, we can greedy mesh -> lets call c the result of &
// 3. the binary rappresentation of c tells us where we can greedy mesh but we wait
// 3a. we need to know if this mesh can keep going to other rows
// 4. bitwise and the next row with c. the result of this & is c'
// 4a. if c != c' -> we cannot keep going. generate this mesh
// 4b. if c == c' -> go to 4
// this is the basic algorithm. it can be optimized but a 
// basic greedy mesher should come out of this

// index referring to the position of a voxel in a column
// 0 => highest voxel
// ... => lower voxel
pub type VoxelIndex = u32;
pub type VoxelNumber = u32;
type VoxelRow = u32;

// voxel position relative to chunk
type VoxelPosition = (usize, usize, usize);

const MAX_HEIGHT: u32 = VoxelRow::BITS - 1;
const CHUNK_ROWS: usize = 32;
const CHUNK_COLS: usize = CHUNK_ROWS;
const VOXEL_SIZE: usize = 1;

// u64 column of binary represented voxels
// 64 x 64 => 64 x 64 x 64 total voxels represented by 1 bit
pub struct Chunk {
    pub voxels: [[VoxelRow ; CHUNK_ROWS] ; CHUNK_COLS],
}

impl Chunk {
    pub fn new() -> Self {
        // TODO: keep assertions if find height is actually used
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

    // returns a vector of begin and end position for the calculated meshes
    // ALGO:
    // 1. select a row
    // 2. generate mesh for all the opaque voxel IN THE SAME u32 (prioritize voxels of the same u32)
    // 3. & the next u32
    // 3a. -> 
    fn generate_quad(&self, y: usize) -> Vec<(VoxelPosition, VoxelPosition)> {
        let row = CHUNK_ROWS - y - 1;
        let mut col = 0;

        loop {
            let mut v1 = self.voxels[row][col];
            v1 = v1 >> v1.trailing_zeros(); // now we know that we have 1 right of the number
            let mask = v1.trailing_ones();

            let mut v2 = self.voxels[row][col+1];
            v2 = v2 >> v2.trailing_zeros(); // now we know that we have 1 right of the number
            let mask2 = v2.trailing_ones();

            if mask == mask2 {
                println!("we can greedy mesh!");
            } else {
                println!("no can do");
            }
        }

    }

    fn get_voxel_position(row: usize, col: usize, slice: usize) -> VoxelPosition {
        (slice * VOXEL_SIZE, row * VOXEL_SIZE, col * VOXEL_SIZE)
    }

    fn generate_meshes(&self) -> Option<Vec<(VoxelPosition, VoxelPosition)>> {
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
