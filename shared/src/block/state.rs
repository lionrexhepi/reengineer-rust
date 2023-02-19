use std::{ fmt::Debug, io::{Write, BufWriter} };

use anyhow::{ ensure };
use bitter::BitReader;
use metrohash::MetroHashMap;
use once_cell::sync::Lazy;
use proc_macros::count_ids;
use serde::{ Serialize, Deserialize, de::Visitor };
use tokio::io::AsyncWriteExt;

use crate::util::block_pos::BlockPos;

use super::simple::*;




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