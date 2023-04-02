use crate::utils::block::Block;

pub struct SavedBlock {
    pub x: i32,
    pub y: i32,
    //Relative position are cartesian coordinates calculated from middle of termgame window
    pub block: Block,
}