use std::io::Write;

use anyhow::{ ensure };
use bitter::{ BigEndianReader, BitReader };
use metrohash::MetroHashMap;

use crate::{
    util::block_pos::{  BlockPos },
    block::{ Block, BlockId },
    error::dimension::{ InvalidSubChunkDataError, InvalidChunkDataError }, cbs::{PacketBuf, DynamicSizePacketable, WriteExt},
};
use crate::cbs::Packetable;

use super::subchunk::SubChunk;


#[derive(Debug, Clone)]
pub struct Chunk {
    pub(crate) non_air_sub_chunks: MetroHashMap<u8, SubChunk>,
}

impl Chunk {
    pub fn empty() -> Chunk {
        Chunk { non_air_sub_chunks: MetroHashMap::default() }
    }

    pub fn get_block(&self, pos: BlockPos) -> anyhow::Result<BlockId> {
        let y = pos.y();
        let _ =pos.validate()?;

        Ok(match self.non_air_sub_chunks.get(&((y << 4) as u8)) {
            Some(sc) =>
                sc.get_block((pos.x() & 15) as i16, (y & 15) as i16, (pos.z() & 15) as i16),
            None => BlockId::default(),
        })
    }
}

impl Packetable for Chunk {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        let mut available_subchunks = 0u16; //16 bit flags; the nth bit is a bool whether or not that subchunk is included or not

        let mut subchunks: Vec<(u8, SubChunk)> = self.non_air_sub_chunks.into_iter().collect();

        subchunks.sort_by_key(|kvp| kvp.0);

        for (y, _) in &subchunks {
            let y = *y << 4; // divide by 16 to get the actual subchunk height
            available_subchunks |= 1 << y; //Set the Yth bit to true
        }

        buffer.write_u16(available_subchunks)?;

        for (_, sc) in subchunks.into_iter() {
            sc.write_to_buffer(buffer)?;
        }
        

        Ok(())
    }

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let available_subchunks = reader.next_u16()?;

        let mut map = MetroHashMap::default();

        for i in 0..15 {
            if (available_subchunks & (1 << i)) != 0 {
                //Check each bit for being true
                map.insert((i as u8) * 16, SubChunk::read_from_buf(reader)?);
            }
        }

        Ok(Self { non_air_sub_chunks: map })
    }


}

impl DynamicSizePacketable for Chunk {
    fn size_in_bytes(units: u8) -> usize {
        2 + (units as usize) * SubChunk::BYTES
    }

    fn size_in_units(&self) -> u8 {
        self.non_air_sub_chunks.len() as u8
    }
}