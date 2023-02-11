use std::{ io::BufWriter, io::Write};

use crate::{block::state::Block, util::WriteExt};

use super::Packetable;

use async_trait::async_trait;

#[async_trait]
impl Packetable for &Block {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_u16(self.to_id())?;
        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let id = reader.next_u16()?;
        Ok(Block::from_id(id)?)
    }
}