use anyhow::anyhow;

#[derive(Debug)]
pub struct InvalidBlockIdError(pub u8);

impl From<InvalidBlockIdError> for anyhow::Error {
    fn from(value: InvalidBlockIdError) -> Self {
        anyhow!("Invalid Block Id: {}", value.0)
    }
}