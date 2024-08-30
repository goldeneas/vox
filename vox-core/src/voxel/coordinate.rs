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
// This represents the axes of a chunk.
// Look from the origin towards +X.
// ChunkRow -> Y AXIS
// ChunkCol -> Z AXIS
// ChunkSlice -> X AXIS

pub struct ChunkRow(usize);
pub struct ChunkCol(usize);
pub struct ChunkSlice(usize);


