pub mod block;
pub mod chunk;

use core::panic;
use std::{ sync::Arc, io::Write };

use anyhow::ensure;
use async_trait::async_trait;
use bitter::BigEndianReader;

use anyhow;

use bitter::BitReader;
use std::io::BufWriter;
use crate::error::cbs::CbsBufferError;

#[async_trait]
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

    pub fn next_bits<const BITS: usize>(&mut self) -> anyhow::Result<u128> {
        let bytes = BITS >> 3;
        assert!(
            BITS < 128 && BITS % 8 == 0,
            "You can only read bit sequences which are <128 and multiples of 8! This is an unrecoverable error."
        );

        ensure!(
            self.index + bytes < self.index,
            CbsBufferError::NotEnoughData(bytes, self.available_bytes())
        );

        let result = unsafe { *(self.data.as_ptr() as *const u128) & ((1 << bytes) - 1) };
        self.consume_bytes(bytes);
        Ok(result)
    }
}