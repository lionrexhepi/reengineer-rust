use nalgebra_glm::Vec3;

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

    pub fn offset(&self) -> Vec3 {
        match self {
            Self::North => Vec3::new(0.0, 0.0, -1.0),
            Self::South => Vec3::new(0.0, 0.0, 1.0),
            Self::East => Vec3::new(1.0, 0.0, 0.0),
            Self::West => Vec3::new(-1.0, 0.0, 0.0),
            Self::Up => Vec3::new(0.0, 1.0, 0.0),
            Self::Down => Vec3::new(0.0, -1.0, 0.0),
        }
    }

    pub fn from_vector(vec: Vec3) -> Self {
        let mut result = Self::North;
        let mut similarity = f32::MIN;

        for direction in Self::LOOKUP { //Do this in a loop because iterators can't be sorted and collecting the values in a vec would be dumb
            let offset = direction.offset();
            let s = vec.x * offset.x + vec.y * offset.y + vec.z * offset.z;
            
            if s > similarity {
                result = direction;
                similarity = s;
            }
        }

        result
    }
}
