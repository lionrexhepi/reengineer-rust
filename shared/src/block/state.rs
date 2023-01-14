use proc_macros::count_ids;

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

impl Block {
    pub fn is_replaceable(&self, pos: (i32, i32, i32)) -> bool {
        self.inner_handler().is_replaceable(pos)
    }
}

pub trait BlockHandler {
    fn is_replaceable(&self, pos: (i32, i32, i32)) -> bool;
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



