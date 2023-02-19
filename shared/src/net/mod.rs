use std::{ fmt::Debug, io::{ Read }, net::TcpStream };

use anyhow::{ bail, ensure, anyhow };
use bitter::{ BigEndianReader, BitReader };
use num_derive::{ FromPrimitive, ToPrimitive };
use num_traits::FromPrimitive;
use uuid::Uuid;

use crate::{
    util::block_pos::{},
    block::{  Block  },
    dimension::chunk::Chunk,
    error::net::PacketReadError,
    cbs::{ Packetable, WriteExt, FixedSizePacketable, DynamicSizePacketable },
};


pub mod packet;
pub mod packet_data;

pub trait NetworkHandler {
    fn enqueue_packet(&self, packet: packet::Packet) -> anyhow::Result<()>;

    fn retrieve_incoming(&mut self) -> Vec<packet::Packet>;

    fn close_all(self);
}