use crate::cbs::{Packetable, PacketBuf, FixedSizePacketable, WriteExt};

#[derive(Debug, Clone)]
pub struct ChunkPos {
    x: i32,
    z: i32,
}

impl ChunkPos {
    pub fn as_long(&self) -> u64 {
        //mostly used as a hash map key
        ((self.x as u64) << 32) | (self.z as u64)
    }

    pub fn new(x: i32, z: i32) -> Self {
        Self { x, z }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn z(&self) -> i32 {
        self.z
    }
}



impl Packetable for ChunkPos {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.first_n_bytes_u32::<24>(self.x() as u32)?;
        buffer.first_n_bytes_u32::<24>(self.z() as u32)?;

        Ok(())
    }

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let x = reader.next_n_bytes_as_u32::<24>()? as i32;
        let z = reader.next_n_bytes_as_u32::<24>()? as i32;

        Ok(Self::new(x, z))
    }

  
}

impl FixedSizePacketable for ChunkPos {
    const SIZE_IN_BYTES: usize = 6;
}
