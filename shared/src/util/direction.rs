use glam::{i32::IVec3, Vec3};

#[repr(i8)]
pub enum AxisDirection {
    Positive = 1,
    Negative = -1,
}

pub enum Axis {
    X,
    Y,
    Z,
}

pub enum Plane {
    Horizontal,
    Vertical,
}

#[repr(u8)]
#[derive(Debug)]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

impl Direction {
    pub const HORIZONTALS: [Self; 4] = [Self::South, Self::East, Self::North, Self::West];

    pub const LOOKUP: [Self; 6] = [
        Self::North,
        Self::South,
        Self::East,
        Self::West,
        Self::Up,
        Self::Down,
    ];

    pub fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }

    pub fn axis(&self) -> Axis {
        match self {
            Self::North | Self::South => Axis::Z,
            Self::East | Self::West => Axis::X,
            Self::Up | Self::Down => Axis::Y,
        }
    }

    pub fn axis_direction(&self) -> AxisDirection {
        match self {
            Self::North | Self::West | Self::Down => AxisDirection::Negative,
            Self::South | Self::East | Self::Up => AxisDirection::Positive,
        }
    }

    pub fn offset(&self) -> IVec3 {
        match self {
            Self::North => IVec3::NEG_Z,
            Self::South => IVec3::Z,
            Self::East => IVec3::X,
            Self::West => -IVec3::NEG_X,
            Self::Up => IVec3::Y,
            Self::Down => -IVec3::NEG_Y,
        }
    }

    pub fn from_vector(vec: Vec3) -> Self {
        let mut result = Self::North;
        let mut similarity = f32::MIN;

        for direction in Self::LOOKUP {
            //Do this in a loop because iterators can't be sorted and collecting the values in a vec would be dumb
            let offset = direction.offset();
    

            let s = vec.x * offset.x as f32 + vec.y * offset.y as f32 + vec.z * offset.z as f32;

            if s > similarity {
                result = direction;
                similarity = s;
            }
        }

        result
    }
}
