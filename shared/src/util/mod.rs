pub mod pos;
pub mod direction;
pub mod movement;

pub mod logger;

pub trait Boxable {
    fn boxed(self) -> Box<Self>;
}

impl<T> Boxable for T {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}