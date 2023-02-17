use std::{ fmt::Debug, io::{ Write, BufWriter } };

use anyhow::{ Ok, bail, ensure };
use bitter::{ BigEndianReader, BitReader };
use uuid::Uuid;

use crate::{
    util::pos::{ BlockPos, ChunkPos },
    block::{ state::{ Block, BlockId } },
    dimension::chunk::Chunk,
    error::net::PacketReadError,
    cbs::{ Packetable, WriteExt, FixedSizePacketable, DynamicSizePacketable, PacketBuf },
};

#[derive(Debug, Clone)]
pub enum PacketData {
    Ping,
    BlockUpdate(BlockPos, BlockId),
    ChunkData(ChunkPos, Chunk),
}

#[repr(u16)]
#[derive(Debug, Clone)]
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
        match self {
            Self::ChunkData => true,
            _ => false,
        }
    }

    pub fn read_data(&self, buf: &mut PacketBuf) -> anyhow::Result<PacketData> {
        match self {
            PacketType::Ping => {
                buf.next_byte();
                Ok(PacketData::Ping)
            }
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
                        ChunkPos::read_from_buf( buf)?,
                        Chunk::read_from_buf(buf)?
                    )
                ),
        }
    }
}

impl PacketData {
    pub fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        match self {
            PacketData::Ping => {
                buffer.write_u8(1)?;
            }
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
}

#[derive(Debug, Clone)]
pub enum PacketDirection {
    FromClient(ClientId),
    FromServer,
    ToClient(ClientId),
    ToServer,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ClientId(Uuid);

impl ClientId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub direction: PacketDirection,
    pub packet_type: PacketType,
    pub data: PacketData,
}

impl Packet {}

pub trait NetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()>;

    fn retrieve_incoming(&mut self) -> Vec<Packet>;

    fn close_all(self);
}