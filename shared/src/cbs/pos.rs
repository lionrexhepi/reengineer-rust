use crate::util::pos::{ ChunkPos, BlockPos };

use super::{ Packetable, WriteExt, FixedSizePacketable };

impl Packetable for ChunkPos {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.first_n_bytes_u32::<24>(self.x() as u32)?;
        buffer.first_n_bytes_u32::<24>(self.z() as u32)?;

        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let x = reader.next_n_bytes_as_u32::<24>()? as i32;
        let z = reader.next_n_bytes_as_u32::<24>()? as i32;

        Ok(Self::new(x, z))
    }

  
}

impl FixedSizePacketable for ChunkPos {
    const SIZE_IN_BYTES: usize = 6;
}

impl Packetable for BlockPos {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u64(self.as_long());
        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self>
        where Self: Sized
    {
        let long = reader.next_u64()?;

        Ok(Self::from_long(long))
    }

    
}

impl FixedSizePacketable for BlockPos {
    const SIZE_IN_BYTES: usize = 8;
}