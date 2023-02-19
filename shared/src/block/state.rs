use std::{ fmt::Debug };









use crate::util::block_pos::BlockPos;






pub trait BlockHandler {
    fn is_replaceable(&self, _pos: BlockPos) -> bool {
        false
    }

    fn map_color(&self) -> i32 {
        1
    }
}

pub trait State: Debug + Clone {
    fn id(&self) -> u8 {
        0
    }
    fn from_id(_id: u8) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self::DEFAULT)
    }
    const DEFAULT: Self;
}