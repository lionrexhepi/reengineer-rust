use std::{ io::Write, slice::from_raw_parts };

use metrohash::MetroHashMap;

use crate::{ dimension::chunk::{ SubChunk, Chunk } };

use super::{ Packetable, WriteExt, FixedSizePacketable, DynamicSizePacketable };

impl Packetable for SubChunk {
    fn write_to_buffer<T: std::io::Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        buffer.write_all(unsafe { from_raw_parts(self.data.as_ptr() as *const u8, Self::BYTES) })?;
        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let bytes = reader.next_bytes::<{ Self::BYTES }>()?;
        Ok(Self {
            data: (unsafe { from_raw_parts(bytes.as_ptr() as *const u16, Self::BLOCK_COUNT) })
                .try_into()
                .unwrap(),
        })
    }

  
    
}

impl FixedSizePacketable for SubChunk {
    const SIZE_IN_BYTES: usize = Self::BYTES;
}

impl Packetable for Chunk {
    fn write_to_buffer<T: Write + Unpin + Send>(
        self,
        buffer: &mut std::io::BufWriter<T>
    ) -> anyhow::Result<()> {
        let mut available_subchunks = 0u16; //16 bit flags; the nth bit is a bool whether or not that subchunk is included or not

        let mut subchunks: Vec<(u8, SubChunk)> = self.non_air_sub_chunks.into_iter().collect();

        subchunks.sort_by_key(|kvp| kvp.0);

        for (y, _) in &subchunks {
            let y = *y << 4; // divide by 16 to get the actual subchunk height
            available_subchunks |= 1 << y; //Set the Yth bit to true
        }

        buffer.write_u16(available_subchunks)?;

        for (_, sc) in subchunks.into_iter() {
            sc.write_to_buffer(buffer)?;
        }
        

        Ok(())
    }

    fn read_from_buf(reader: &mut super::PacketBuf) -> anyhow::Result<Self> where Self: Sized {
        let available_subchunks = reader.next_u16()?;

        let mut map = MetroHashMap::default();

        for i in 0..15 {
            if (available_subchunks & (1 << i)) != 0 {
                //Check each bit for being true
                map.insert((i as u8) * 16, SubChunk::read_from_buf(reader)?);
            }
        }

        Ok(Self { non_air_sub_chunks: map })
    }


}

impl DynamicSizePacketable for Chunk {
    fn size_in_bytes(units: u8) -> usize {
        2 + (units as usize) * SubChunk::BYTES
    }

    fn size_in_units(&self) -> u8 {
        self.non_air_sub_chunks.len() as u8
    }
}