use super::state::{State, BlockHandler};

pub struct AirState;

impl State for AirState {
    const DEFAULT: Self = Self;
}

impl BlockHandler for AirState {
    fn is_replaceable(&self, _pos: (i32, i32, i32)) -> bool {
        true
    }
}


pub struct GrassState {
    snowy: bool,
}

impl GrassState {
    pub const NORMAL: Self = Self { snowy: false };
    pub const SNOWY: Self = Self { snowy: true };
}

impl BlockHandler for GrassState {
    fn is_replaceable(&self, _pos: (i32, i32, i32)) -> bool {
        false
    }
}

impl State for GrassState {
    fn id(&self) -> u8 {
        self.snowy as u8
    }

    fn from_id(id: u8) -> Self {
        Self { snowy: id == 1 }
    }

    const DEFAULT: Self = Self::NORMAL;
}
