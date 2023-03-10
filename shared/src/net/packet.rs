use std::io::ErrorKind;
use std::io::Read;

use crate::cbs::WriteExt;
use crate::error::net::PacketReadError;

use num_traits::FromPrimitive;
use num_traits::ToPrimitive;

use std::net::TcpStream;

use uuid::Uuid;

use std::io::BufWriter;

use std::io::Write;

use anyhow::{ anyhow, Ok };

use crate::cbs::PacketBuf;

use super::packet_data::PacketData;
use super::packet_data::PacketType;

#[derive(Debug, Clone)]
pub enum PacketDirection {
    FromClient(ClientId),
    FromServer,
    ToClient(ClientId),
    ToServer,
}


#[derive(Debug, Clone)]
pub enum PacketSource {
    Server,
    Client(ClientId)
}

impl PacketSource {
    pub fn as_direction(self) -> PacketDirection {
        match self {
            PacketSource::Server => PacketDirection::FromServer,
            PacketSource::Client(id) => PacketDirection::FromClient(id),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct ClientId(Uuid);

impl Default for ClientId {
     fn default() -> Self {
        Self(Uuid::nil())
    }
}

impl ClientId {
    pub fn new() -> ClientId {
        ClientId(Uuid::new_v4())
    }
}

#[derive(Debug, Clone)]
pub struct Packet {
    pub direction: PacketDirection,
    pub packet_type: PacketType,
    pub data: PacketData,
}

impl Packet {
    pub fn try_from_stream(stream: &mut TcpStream, source: PacketSource) -> anyhow::Result<Option<Packet>> {
        let mut incoming_type = [0u8; 2];

        match stream.read_exact(&mut incoming_type) {
            Result::Ok(_) => {
                let incoming_type = u16::from_le_bytes(incoming_type);
                let packet_type = <PacketType as FromPrimitive>
                    ::from_u16(incoming_type)
                    .ok_or(PacketReadError::InvalidPacketType(incoming_type))?;
                let mut size = None;

                if packet_type.size_can_vary() {
                    let mut packet_size_buf = [0u8; 1];

                    stream.read_exact(&mut packet_size_buf)?;

                    size = Some(packet_size_buf[0]);
                }

                let size = packet_type.get_required_buffer_size(size);

                let mut data_buffer = vec![0u8; size];

                stream.read_exact(&mut data_buffer[0..])?;

                let mut buffer = PacketBuf::new(data_buffer.into_boxed_slice());

                let data = PacketData::read_data(packet_type, &mut buffer)?;

                Ok(Some(Packet { direction: source.as_direction(), packet_type, data }))
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => { Ok(None) /*No bytes received*/ }
            Err(e) => { Err(anyhow!(e)) }
        }
    }

    pub fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u16(<PacketType as ToPrimitive>::to_u16(&self.packet_type).unwrap())?;

        if let Some(header) = self.data.size_header() {
            buffer.write_u8(header)?;
        }

        self.data.write_to_buffer(buffer)?;

        Ok(())
    }
}