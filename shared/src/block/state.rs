use proc_macros::count_ids;

#[count_ids]
pub enum Block {
    Air(AirState),
    Grass(GrassState),
}

impl Block {
    pub fn to_id(&self) -> u16 {
        (self.repr() as u16) << 8 | self.variant_id() as u16
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
