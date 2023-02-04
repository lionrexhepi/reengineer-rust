use std::{ fmt::Debug };

use anyhow::{ Ok, anyhow, bail, ensure };
use bitter::{ BigEndianReader, BitReader };
use tokio::{ io::{ BufWriter, AsyncWriteExt }, io::{ AsyncWrite, self } };
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

    fn read_from_bytes(reader: &mut BigEndianReader) -> anyhow::Result<Self> where Self: Sized;
}

pub enum PacketReadError {
    InvalidPacketType(u16),
    NotEnoughData(u32, u32),
    IOError(io::Error),
}

impl From<PacketReadError> for anyhow::Error {
    fn from(value: PacketReadError) -> Self {
        match value {
            PacketReadError::NotEnoughData(actual, minimum) =>
                anyhow!(
                    "Packet data too small! Read {} bits while at least {} were needed",
                    actual,
                    minimum
                ),
            PacketReadError::IOError(inner) => inner.into(),
            PacketReadError::InvalidPacketType(id) =>
                anyhow!("There is no Packet type with ID {}!", id),
        }
    }
}

#[repr(u16)]
#[derive(Debug, Clone)]
pub enum PacketData {
    Ping,
    BlockUpdate(BlockPos, &'static Block),
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
        let mut reader = BigEndianReader::new(data);

        let len = reader.refill_lookahead();
        ensure!(len >= 16, PacketReadError::NotEnoughData(len, 16));

        let id = reader.peek(16) as u16;
        reader.consume(16);

        match id {
            0 => {
                let len = reader.refill_lookahead();
                ensure!(len >= 8, PacketReadError::NotEnoughData(len, 8));
                reader.consume(8);
                Ok(Self::Ping)
            }
            1 => {
                Ok(
                    Self::BlockUpdate(
                        BlockPos::read_from_bytes(&mut reader)?,
                        <&Block>::read_from_bytes(&mut reader)?
                    )
                )
            }

            _ => bail!(PacketReadError::InvalidPacketType(id)),
        }
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