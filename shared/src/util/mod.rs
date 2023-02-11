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

pub trait WriteExt: Write {
    fn write_u8(&mut self, val: u8) -> anyhow::Result<usize> {
        Ok(self.write(&[val])?)
    }

    fn write_u16(&mut self, val: u16) -> anyhow::Result<usize> {
        let buf = [0u8; 2];

        unsafe {
            *(buf.as_mut_ptr() as *mut u16) = val;
        }

        Ok(self.write(&buf)?)
    }

    fn write_u32(&mut self, val: u32) -> anyhow::Result<usize> {
        let buf = [0u8; 4];

        unsafe {
            *(buf.as_mut_ptr() as *mut u32) = val;
        }

        Ok(self.write(&buf)?)
    }

    fn write_u64(&mut self, val: u64) -> anyhow::Result<usize> {
        let buf = [0u8; 8];

        unsafe {
            *(buf.as_mut_ptr() as *mut u64) = val;
        }

        Ok(self.write(&buf)?)
    }
}

impl<T> WriteExt for T where T: Write {}