pub mod block;
pub mod chunk;
pub mod pos;

use core::panic;
use std::{ sync::Arc, io::Write, any };

use anyhow::ensure;
use async_trait::async_trait;
use bitter::BigEndianReader;

use anyhow;

use bitter::BitReader;
use std::io::BufWriter;
use crate::error::cbs::CbsBufferError;


pub trait Packetable {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()>;

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> where Self: Sized;
}

pub struct PacketBuf<'a> {
    data: Box<[u8]>,
    index: usize,
    total_bytes: usize,
    reader: BigEndianReader<'a>,
}

impl<'a> PacketBuf<'a> {
    pub fn new(data: Box<[u8]>) -> Self {
        Self {
            data,
            index: 0,
            total_bytes: data.len(),
            reader: BigEndianReader::new(&data),
        }
    }

    fn consume_bytes(&mut self, bytes: usize) {
        self.index = self.index + bytes;
    }

    pub fn next_bytes<'b, const BYTES: usize>(&'b mut self) -> anyhow::Result<&'b [u8]> {
        ensure!(
            self.index + BYTES <= self.total_bytes,
            CbsBufferError::NotEnoughData(BYTES, self.available_bytes())
        );

        let result = &self.data[self.index..self.index + BYTES];
        self.consume_bytes(BYTES);
        Ok(&result)
    }

    pub fn available_bytes(&self) -> usize {
        self.total_bytes - self.index
    }

    pub fn next_byte(&mut self) -> anyhow::Result<u8> {
        ensure!(
            self.index < self.total_bytes,
            CbsBufferError::NotEnoughData(1, self.available_bytes())
        );

        let result = self.data[self.index >> 3];

        self.consume_bytes(1);

        Ok(result)
    }

    pub fn next_u16(&mut self) -> anyhow::Result<u16> {
        ensure!(
            self.index + 2 <= self.total_bytes,
            CbsBufferError::NotEnoughData(2, self.available_bytes())
        );

        let result = unsafe { *(self.data.as_ptr() as *const u16) };

        self.consume_bytes(2);

        Ok(result)
    }

    pub fn next_u32(&mut self) -> anyhow::Result<u32> {
        ensure!(
            self.index + 4 <= self.total_bytes,
            CbsBufferError::NotEnoughData(4, self.available_bytes())
        );

        let result = unsafe { *(self.data.as_ptr() as *const u32) };

        self.consume_bytes(4);

        Ok(result)
    }

    pub fn next_u64(&mut self) -> anyhow::Result<u64> {
        ensure!(
            self.index + 8 <= self.total_bytes,
            CbsBufferError::NotEnoughData(8, self.available_bytes())
        );

        let result = unsafe { *(self.data.as_ptr() as *const u64) };

        self.consume_bytes(8);

        Ok(result)
    }

    fn next_n_bytes_as_u32<const BYTES: usize>(&mut self) -> anyhow::Result<u32> {

        let mut temp = [0u8; 4];

        temp.copy_from_slice(self.next_bytes::<BYTES>()?);

        Ok(u32::from_le_bytes(temp))
    }
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

    fn first_n_bytes_u128<const BYTES: usize>(&mut self, int: u128) -> anyhow::Result<usize> {
        assert!(BYTES <= 16);
        Ok(self.write(&int.to_le_bytes()[0..BYTES])?)
    }

    fn first_n_bytes_u32<const BYTES: usize>(&mut self, int: u32) -> anyhow::Result<usize> {
        assert!(BYTES <= 4);
        Ok(self.write(&int.to_le_bytes()[0..BYTES])?)
    }
}

impl<T> WriteExt for T where T: Write {}