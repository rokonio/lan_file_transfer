use anyhow::{bail, Result};
use num_enum::TryFromPrimitive;

#[derive(Clone, Copy, Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Operation {
    StartSendFile = 20,
    SendFileContent = 21,
    EndSendFile = 22,

    RequestSucces = 200,
    RequestRefuse = 202,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Header {
    pub operation: Operation,
    pub length: u32,
}

impl Header {
    pub const SIZE: usize = 5;
    pub fn encode(&self) -> [u8; Self::SIZE] {
        let mut buf = [0; Self::SIZE];
        buf[0] = self.operation as u8;
        let length_bytes = self.length.to_be_bytes();
        buf[1] = length_bytes[0];
        buf[2] = length_bytes[1];
        buf[3] = length_bytes[2];
        buf[4] = length_bytes[3];
        buf
    }

    pub fn decode(buf: [u8; Self::SIZE]) -> Result<Self> {
        let operation = Operation::try_from(buf[0])?;
        let Some([l1, l2, l3, l4]) = buf.get(1..5) else {
            bail!("Couldnt' parse incoming header");
        };
        let length = u32::from_be_bytes([*l1, *l2, *l3, *l4]);
        Ok(Self { operation, length })
    }
}
