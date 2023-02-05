use std::io;
use anyhow::anyhow;

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