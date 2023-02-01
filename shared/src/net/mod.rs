use std::{ net::TcpStream, fmt::Debug };

use tokio::{ net::unix::SocketAddr, io::BufWriter };
use uuid::Uuid;

use crate::{
    util::pos::{ BlockPos, ChunkPos },
    block::{ state::{ Block, State }, simple::AirState },
    dimension::chunk::Chunk,
};

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

    pub async fn write_to_buffer<T>(self, buffer: &BufWriter<T>) -> anyhow::Result<()> {
        todo!()
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