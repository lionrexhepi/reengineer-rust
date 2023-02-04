use std::{ fmt::Debug, collections::hash_map::Entry };

use bitter::BitReader;
use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
use proc_macros::count_ids;
use log::error;
use tokio::io::AsyncWriteExt;

use crate::{ util::pos::BlockPos, net::Packetable, wait };

use super::simple::*;

static mut CACHE: Lazy<MetroHashMap<u16, Block>> = Lazy::new(MetroHashMap::default);

#[count_ids]
#[derive(Debug, Clone)]
pub enum Block {
    Air(AirState),
    Grass(GrassState),
}

fn get_cache() -> &'static mut Lazy<MetroHashMap<u16, Block>> {
    unsafe {
        &mut CACHE
    } //race conditions etc shouldnt be an issue since even if two block states are inserted at the same time, theyre the same anyway
}

impl Block {
    pub fn to_id(&self) -> u16 {
        ((self.repr() as u16) << 8) | (self.variant_id() as u16)
    }

    pub fn from_id(id: u16) -> Option<&'static Self> {
        let map = get_cache();

        let entry = map.entry(id);

        if let Entry::Vacant(_) = entry {
            if let Some(new) = Self::from_ints((id >> 8) as u8, (id & 255) as u8) {
                Some(entry.or_insert(new))
            } else {
                error!("Invalid block id: {}", id);
                Some(<&Block>::default())
            }
        } else {
            Some(
                entry.or_insert_with(||
                    panic!(
                        "Logically and factually this line should never have been called. You failed."
                    )
                )
            )
        }
    }
}

impl Default for &Block {
    fn default() -> Self {
        (unsafe { &mut CACHE }).entry(0).or_insert(Block::Air(AirState::DEFAULT))
    }
}

impl BlockHandler for Block {
    fn is_replaceable(&self, pos: BlockPos) -> bool {
        self.inner_handler().is_replaceable(pos)
    }
}

impl Packetable for &Block {
    fn write_to_buffer<T: tokio::io::AsyncWrite + Unpin>(
        self,
        buffer: &mut tokio::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        wait!(buffer.write_u16(self.to_id()))?;
        Ok(())
    }

    fn read_from_bytes(reader: &mut bitter::BigEndianReader) -> Option<Self> {
        let len = reader.refill_lookahead();
        assert!(len >= 16);
        let id = reader.peek(16) as u16;
        reader.consume(16);
        Block::from_id(id)
    }
}

pub trait BlockHandler {
    fn is_replaceable(&self, _pos: BlockPos) -> bool {
        false
    }

    fn map_color(&self) -> i32 {
        1
    }
}

pub trait State: Debug + Clone {
    fn id(&self) -> u8 {
        0
    }
    fn from_id(_id: u8) -> Self where Self: Sized {
        Self::DEFAULT
    }
    const DEFAULT: Self;
}