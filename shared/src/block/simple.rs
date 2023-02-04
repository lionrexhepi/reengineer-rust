use crate::util::pos::BlockPos;

use super::state::{ BlockHandler, State };

#[derive(Debug, Clone)]
pub struct AirState;

impl State for AirState {
    const DEFAULT: Self = Self;
}

impl BlockHandler for AirState {
    fn is_replaceable(&self, _pos: BlockPos) -> bool {
        true
    }
}

#[derive(Debug, Clone)]
pub struct GrassState {
    snowy: bool,
}

impl GrassState {
    pub const NORMAL: Self = Self { snowy: false };
    pub const SNOWY: Self = Self { snowy: true };
}

impl BlockHandler for GrassState {}

impl State for GrassState {
    fn id(&self) -> u8 {
        self.snowy as u8
    }

    fn from_id(id: u8) -> anyhow::Result<Self> {
        Ok(Self { snowy: id == 1 })
    }

    const DEFAULT: Self = Self::NORMAL;
}