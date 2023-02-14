use std::io::Write;

pub mod pos;
pub mod direction;

pub mod logger;

pub trait Boxable {
    fn boxed(self) -> Box<Self>;
}

impl<T> Boxable for T {
    fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}

#[macro_export]
macro_rules! wait {
    ($future:expr) => {
        (futures::executor::block_on($future))
    };
}

