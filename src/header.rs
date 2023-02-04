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
    pub id: u16,
}

impl Header {
    pub const SIZE: usize = 7;
    pub fn encode(&self) -> [u8; Self::SIZE] {
        let mut buf = [0; Self::SIZE];
        buf[0] = self.operation as u8;
        let length_bytes = self.length.to_be_bytes();
        buf[1] = length_bytes[0];
        buf[2] = length_bytes[1];
        buf[3] = length_bytes[2];
        buf[4] = length_bytes[3];
        let id_bytes = self.id.to_be_bytes();
        buf[5] = id_bytes[0];
        buf[6] = id_bytes[1];
        buf
    }

    pub fn decode(
        buf: [u8; Self::SIZE],
    ) -> Result<Self, num_enum::TryFromPrimitiveError<Operation>> {
        let [b0, b1, b2, b3, b4, b5, b6] = buf;
        let operation = Operation::try_from(b0)?;
        let length = u32::from_be_bytes([b1, b2, b3, b4]);
        let id = u16::from_be_bytes([b5, b6]);
        Ok(Self {
            operation,
            length,
            id,
        })
    }
}
