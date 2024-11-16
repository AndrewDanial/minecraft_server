use crate::varint::VarInt;
#[derive(Debug, Clone, Default)]
pub struct SetCenterChunk {
    pub chunk_x: VarInt,
    pub chunk_z: VarInt,
}

impl SetCenterChunk {
    const PROTOCOL_ID: u8 = 0x58;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.extend_from_slice(&self.chunk_x.write_varint());
        vec.extend_from_slice(&self.chunk_z.write_varint());
        vec.insert(0, vec.len() as u8);
        vec
    }
}
