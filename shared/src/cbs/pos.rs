use crate::util::pos::ChunkPos;

use super::{ Packetable, WriteExt };

impl Packetable for ChunkPos {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.first_n_bytes_u32::<24>(self.x() as u32);
        buffer.first_n_bytes_u32::<24>(self.z() as u32);

        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        
        let x = reader.next_n_bytes_as_u32::<24>()? as i32;
        let z = reader.next_n_bytes_as_u32::<24>()? as i32;

        Ok(Self {
            x, z
        })

    }
}