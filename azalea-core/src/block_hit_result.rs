use crate::{BlockPos, Direction, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct BlockHitResult {
    pub location: Vec3,
    pub direction: Direction,
    pub block_pos: BlockPos,
    pub miss: bool,
    pub inside: bool,
}
