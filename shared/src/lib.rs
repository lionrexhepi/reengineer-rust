pub mod block;
#[macro_use]
pub mod util;
pub mod dimension;
pub mod net;

#[cfg(test)]
mod test {
    use crate::util::{ pos::BlockPos };

    #[test]
    pub fn test_block_pos() {
        let pos = BlockPos::new(32, 6, 788);
        let long = pos.as_long();
        let new_pos = BlockPos::from_long(long);

        assert_eq!(pos, new_pos);
    }
}