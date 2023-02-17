use std::{ io::BufWriter, io::Write };

use crate::{ block::state::BlockId };

use super::{ Packetable, PacketBuf, WriteExt, FixedSizePacketable };

impl FixedSizePacketable for BlockId {
    const SIZE_IN_BYTES: usize = 2;
}

impl Packetable for BlockId {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u16(self.0)?;
        Ok(())
    }

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> {
        let id = reader.next_u16()?;
        Ok(Self(id))
    }
}