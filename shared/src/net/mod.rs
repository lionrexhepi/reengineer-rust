use crate::{util::pos::{BlockPos, ChunkPos}, block::{state::{Block, State}, simple::AirState}, dimension::chunk::Chunk};

#[repr(u16)]
#[derive(Debug)]
pub enum PacketData {
    Ping,
    BlockUpdate(BlockPos, Block),
    ChunkData(ChunkPos, Chunk)
}

impl PacketData {
    fn discriminant(&self) -> u16 {
        unsafe { *<*const _>::from(self).cast::<u16>() }
    }
}

#[derive(Debug)]
pub enum PacketDirection {
    Serverbound,
    Clientbound
}

#[derive(Debug)]
pub struct Packet {
    pub direction: PacketDirection,
    pub data: PacketData 
}



pub trait NetworkHandler {
    fn enqueue_packet(&self, packet: Packet) -> anyhow::Result<()>;

    fn retrieve_incoming(&self) -> Vec<Packet>;
}