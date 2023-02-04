use std::{ fmt::Debug };

use anyhow::Ok;
use bitter::BigEndianReader;
use futures::executor::block_on;
use tokio::{ io::{ BufWriter, AsyncWriteExt }, io::AsyncWrite };
use uuid::Uuid;

use crate::{
    util::pos::{ BlockPos, ChunkPos },
    block::{ state::{ Block } },
    dimension::chunk::Chunk,
};
pub trait Packetable {
    fn write_to_buffer<T: AsyncWrite + Unpin>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()>;

    fn read_from_bytes(reader: &mut BigEndianReader) -> Option<Self> where Self:Sized;
}

#[repr(u16)]
#[derive(Debug, Clone)]
pub enum PacketData {
    Ping,
    BlockUpdate(BlockPos, Block),
    ChunkData(ChunkPos, Chunk),
}

impl PacketData {
    fn discriminant(&self) -> u16 {
        unsafe { *<*const _>::from(self).cast::<u16>() }
    }

    pub async fn write_to_buffer<T: AsyncWrite + Unpin>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u16(self.discriminant()).await?;

        match self {
            PacketData::Ping => buffer.write_u8(1).await?,
            PacketData::BlockUpdate(pos, block) => {
                pos.write_to_buffer(buffer)?;
                block.write_to_buffer(buffer)?;
            }
            PacketData::ChunkData(pos, _chunk) => {
                buffer.write_u64(pos.as_long()).await?; //TODO: write the chunk too
            }
        }

        Ok(())
    }

    pub fn from_bytes(data: &[u8]) -> anyhow::Result<Self> {
        todo!()
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
    pub data: PacketData,
}

impl Packet {}

pub trait NetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()>;

    fn retrieve_incoming(&mut self) -> Vec<Packet>;

    fn close_all(self);
}