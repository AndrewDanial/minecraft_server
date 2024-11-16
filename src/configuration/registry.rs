use crate::varint::VarInt;
use crate::NBT;
#[derive(Debug, Clone)]

pub struct RegistryDataPacket<'a> {
    pub registry_id: &'a str,
    pub entry_count: u8,
    pub entries: Vec<RegistryData<'a>>,
}

#[derive(Debug, Clone)]
pub struct RegistryData<'a> {
    pub entry_id: &'a str,
    pub has_data: bool,
    pub data: Option<NBT>,
}

impl<'a> RegistryDataPacket<'a> {
    const PROTOCOL_ID: u8 = 0x07;
    pub fn new(registry_id: &'a str, entry_count: u8, entries: Vec<RegistryData<'a>>) -> Self {
        Self {
            registry_id,
            entry_count,
            entries,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.push(self.registry_id.len() as u8);
        vec.extend_from_slice(self.registry_id.as_bytes());
        vec.push(self.entry_count);
        for i in &self.entries {
            vec.append(&mut i.to_bytes());
        }

        let mut vec_len = VarInt(vec.len() as u64).write_varint();
        vec_len.append(&mut vec);
        vec_len
    }
}

impl<'a> RegistryData<'a> {
    fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(self.entry_id.len() as u8);
        vec.extend_from_slice(self.entry_id.as_bytes());
        vec.push(self.has_data as u8);
        if self.has_data {
            let data = self.data.clone().unwrap();
            vec.extend_from_slice(&NBT::tag_to_bytes(data));
        }
        vec
    }
}
