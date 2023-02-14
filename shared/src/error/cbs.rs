use anyhow::anyhow;

pub enum CbsBufferError {
    NotEnoughData(usize, usize)
}

impl From<CbsBufferError> for anyhow::Error {
    fn from(value: CbsBufferError) -> Self {
        match  value {
            CbsBufferError::NotEnoughData(needed, present) => anyhow!("Only {} bits remaining in buffer, however this read operation wants to read {} bits", present, needed),
        }
    }
}