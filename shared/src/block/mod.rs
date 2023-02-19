pub mod state;
pub mod simple;

use std::io::{ Write, BufWriter };

use proc_macros::count_ids;

use crate::{
    util::block_pos::BlockPos,
    error::{ block::* },
    cbs::{ Packetable, FixedSizePacketable, PacketBuf, WriteExt },
};

use self::{simple::*, state::*, state::BlockHandler};

mod cache {
    use metrohash::MetroHashMap;
    use once_cell::sync::Lazy;

    use super::Block;

    static mut CACHE: Lazy<MetroHashMap<u16, Block>> = Lazy::new(MetroHashMap::default);

    pub fn get_cache() -> &'static mut Lazy<MetroHashMap<u16, Block>> {
        unsafe {
            &mut CACHE
        } //race conditions etc shouldnt be an issue since even if two block states are inserted at the same time, theyre the same anyway
    }
}

#[count_ids]
#[derive(Debug, Clone)]
pub enum Block {
    Air(AirState),
    Grass(GrassState),
}

impl Block {
    pub fn to_id(&self) -> BlockId {
        BlockId(((self.repr() as u16) << 8) | (self.variant_id() as u16))
    }
}

impl Default for &Block {
    fn default() -> Self {
        cache::get_cache().entry(0).or_insert(Block::Air(AirState::DEFAULT))
    }
}

impl BlockHandler for Block {
    fn is_replaceable(&self, pos: BlockPos) -> bool {
        self.inner_handler().is_replaceable(pos)
    }
}


unsafe impl Send for Block {}

#[derive(Default, Debug, Clone, Copy)]
pub struct BlockId(pub u16);

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

impl BlockId {
    pub fn resolve(&self) -> anyhow::Result<&'static Block> {
        let map = cache::get_cache();

        if let std::collections::hash_map::Entry::Vacant(e) = map.entry(self.0) {
            e.insert(Block::from_ints((self.0 >> 8) as u8, (self.0 & 255) as u8)?);
        };

        Ok(
            map
                .get(&self.0)
                .expect("This is impossible as the value either existed or was just inserted.")
        )
    }
}