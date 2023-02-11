use std::{ fmt::Debug };

use anyhow::{ ensure };
use bitter::BitReader;
use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
use proc_macros::count_ids;
use serde::{Serialize, Deserialize, de::Visitor};
use tokio::io::AsyncWriteExt;

use crate::{
    util::pos::BlockPos,
    wait,
    error::{ block::*, net::PacketReadError },
    cbs::Packetable,
};

use super::simple::*;

static mut CACHE: Lazy<MetroHashMap<u16, Block>> = Lazy::new(MetroHashMap::default);

#[count_ids]
#[derive(Debug, Clone)]
pub enum Block {
    Air(AirState),
    Grass(GrassState),
}

unsafe impl Send for Block {}

fn get_cache() -> &'static mut Lazy<MetroHashMap<u16, Block>> {
    unsafe {
        &mut CACHE
    } //race conditions etc shouldnt be an issue since even if two block states are inserted at the same time, theyre the same anyway
}

impl Block {
    pub fn to_id(&self) -> u16 {
        ((self.repr() as u16) << 8) | (self.variant_id() as u16)
    }

    pub fn from_id(id: u16) -> anyhow::Result<&'static Self> {
        let map = get_cache();

        Ok(map.entry(id).or_insert(Self::from_ints((id >> 8) as u8, (id & 255) as u8)?))
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
    fn from_id(_id: u8) -> anyhow::Result<Self> where Self: Sized {
        Ok(Self::DEFAULT)
    }
    const DEFAULT: Self;
}

