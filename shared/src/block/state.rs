use proc_macros::count_ids;

use crate::util::block_pos::BlockPos;

use super::simple::*;

#[count_ids]
pub enum Block {
    Air(AirState),
    Grass(GrassState),
}

impl Block {
    pub fn to_id(&self) -> u16 {
        (self.repr() as u16) << 8 | self.variant_id() as u16
    }

    pub fn from_id(id:u16) -> Option<Self> {
        Self::from_ints((id >> 8) as u8, (id & 255) as u8)
    }
}

impl BlockHandler for Block {
     fn is_replaceable(&self, pos: BlockPos) -> bool {
        self.inner_handler().is_replaceable(pos)
    }
}

pub trait BlockHandler {
    fn is_replaceable(&self, pos: BlockPos) -> bool {
        false
    }
}

pub trait State {
    fn id(&self) -> u8 {
        0
    }
    fn from_id(_id: u8) -> Self
    where
        Self: Sized,
    {
        Self::DEFAULT
    }
    const DEFAULT: Self;
}



