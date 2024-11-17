use crate::light::Light;
use crate::nbt::NBT;
use crate::varint::VarInt;
#[derive(Debug, Clone)]
pub struct ChunkData {
    pub chunk_x: i32,
    pub chunk_z: i32,
    pub heightmaps: NBT,
    pub size: VarInt,
    pub data: Vec<u8>,
    pub num_of_block_entities: VarInt,
    pub block_entities: Vec<BlockEntity>,
    pub light_data: Light,
}

impl ChunkData {
    const PROTOCOL_ID: u8 = 0x28;

    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            chunk_x,
            chunk_z,
            heightmaps: NBT::TagCompound("", vec![]),
            size: VarInt(0),
            data: vec![],
            num_of_block_entities: VarInt(0),
            block_entities: vec![],
            light_data: Light::new(),
        }
    }
    pub fn to_bytes(&self, stone: &str) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.extend_from_slice(&self.chunk_x.to_be_bytes());
        vec.extend_from_slice(&self.chunk_z.to_be_bytes());
        vec.extend_from_slice(&NBT::tag_to_bytes(self.heightmaps.clone()));
        // vec.extend_from_slice(&self.size.write_varint());
        let mut dummy_data: Vec<u8> = vec![];

        match stone {
            "stone" => dummy_data = Self::stone_data(),
            "1" => dummy_data = Self::something_1(),
            "2" => dummy_data = Self::something_2(),
            "air" => dummy_data = Self::air_data(),
            _ => unimplemented!(),
        }
        vec.extend_from_slice(&VarInt(dummy_data.len() as u64 * 24).write_varint());

        for _ in 0..12 {
            vec.extend_from_slice(&dummy_data);
        }
        for _ in 0..12 {
            vec.extend_from_slice(&Self::air_data());
        }
        vec.extend_from_slice(&self.num_of_block_entities.write_varint());
        for i in &self.block_entities {
            vec.extend_from_slice(&i.to_bytes());
        }
        vec.extend_from_slice(&self.light_data.to_bytes());
        let mut len = VarInt(vec.len() as u64).write_varint();
        len.append(&mut vec);
        len
    }

    pub fn stone_data() -> Vec<u8> {
        vec![
            0xFF, 0xFF, 0x00, 0x01, 0x00, 0x1, 0x2, 0x00, 0x01, 0x01, 0xCC, 0xFF, 0xCC, 0xFF, 0xCC,
            0xFF, 0xCC, 0xFF,
        ]
    }

    pub fn air_data() -> Vec<u8> {
        vec![
            0xFF, 0xFF, 0x00, 0x00, 0x00, 0x1, 0x2, 0x00, 0x01, 0x01, 0xCC, 0xFF, 0xCC, 0xFF, 0xCC,
            0xFF, 0xCC, 0xFF,
        ]
    }

    pub fn something_1() -> Vec<u8> {
        vec![
            0xFF, 0xFF, 0x00, 0x02, 0x00, 0x1, 0x2, 0x00, 0x01, 0x01, 0xCC, 0xFF, 0xCC, 0xFF, 0xCC,
            0xFF, 0xCC, 0xFF,
        ]
    }

    pub fn something_2() -> Vec<u8> {
        vec![
            0xFF, 0xFF, 0x00, 0x16, 0x00, 0x1, 0x2, 0x00, 0x01, 0x01, 0xCC, 0xFF, 0xCC, 0xFF, 0xCC,
            0xFF, 0xCC, 0xFF,
        ]
    }
}

#[derive(Default, Debug, Clone)]
pub struct BlockEntity {
    pub packed_xz: u8,
    pub y: u16,
    pub block_type: VarInt,
    pub data: NBT,
}

impl BlockEntity {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(self.packed_xz);
        vec.extend_from_slice(&self.y.to_be_bytes());
        vec.extend_from_slice(&self.block_type.write_varint());
        vec.extend_from_slice(&NBT::tag_to_bytes(NBT::TagCompound("", vec![])));
        vec
    }
}
