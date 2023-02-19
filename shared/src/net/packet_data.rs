use anyhow::Ok;

use anyhow;
use num_derive::FromPrimitive;
use num_derive::ToPrimitive;

use std::io::BufWriter;

use std::io::Write;

use crate::cbs::DynamicSizePacketable;
use crate::cbs::FixedSizePacketable;
use crate::cbs::PacketBuf;
use crate::cbs::Packetable;
use crate::dimension::chunk::Chunk;


use crate::block::BlockId;

use crate::util::block_pos::BlockPos;
use crate::util::chunk_pos::ChunkPos;

#[derive(Debug, Clone)]
pub enum PacketData {
    Ping,
    BlockUpdate(BlockPos, BlockId),
    ChunkData(ChunkPos, Chunk),
}

impl PacketData {
    pub fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        match self {
            PacketData::Ping => {}
            PacketData::BlockUpdate(pos, block) => {
                pos.write_to_buffer(buffer)?;
                block.write_to_buffer(buffer)?;
            }
            PacketData::ChunkData(pos, chunk) => {
                pos.write_to_buffer(buffer)?;
                chunk.write_to_buffer(buffer)?;
            }
        }

        Ok(())
    }

    pub fn size_header(&self) -> Option<u8> {
        match self  {
            PacketData::ChunkData(_, chunk) => Some(chunk.size_in_units()),
            _ => None
        }
    }

    pub fn read_data(packet_type: PacketType, buf: &mut PacketBuf) -> anyhow::Result<PacketData> {
        match packet_type {
            PacketType::Ping => { Ok(PacketData::Ping) }
            PacketType::BlockUpdate =>
                Ok(
                    PacketData::BlockUpdate(
                        BlockPos::read_from_buf(buf)?,
                        BlockId::read_from_buf(buf)?
                    )
                ),
            PacketType::ChunkData =>
                Ok(
                    PacketData::ChunkData(
                        ChunkPos::read_from_buf(buf)?,
                        Chunk::read_from_buf(buf)?
                    )
                ),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone, Copy, FromPrimitive, ToPrimitive)]
pub enum PacketType {
    Ping,
    BlockUpdate,
    ChunkData,
}

impl PacketType {
    pub fn get_required_buffer_size(&self, size_header: Option<u8>) -> usize {
        match self {
            PacketType::Ping => 1,
            PacketType::BlockUpdate => BlockPos::SIZE_IN_BYTES + BlockId::SIZE_IN_BYTES,
            PacketType::ChunkData =>
                ChunkPos::SIZE_IN_BYTES +
                    Chunk::size_in_bytes(size_header.expect("This should never happen.")),
        }
    }

    pub fn size_can_vary(&self) -> bool {
        matches!(self, Self::ChunkData)
    }

    
}