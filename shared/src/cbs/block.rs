use std::{ io::BufWriter, io::Write };

use crate::{ block::state::BlockId, util::WriteExt };

use super::Packetable;

impl Packetable for BlockId {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut BufWriter<T>
    ) -> anyhow::Result<()> {
       buffer.write_u16(self.0) 
    };

    fn read_from_buf(reader: &mut PacketBuf) -> anyhow::Result<Self> where Self: Sized;


}