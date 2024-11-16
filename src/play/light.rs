use crate::bitset::BitSet;
use crate::nbt::NBT;
use crate::varint::VarInt;
#[derive(Default, Clone, Debug)]
pub struct Light {
    pub sky_light_mask: BitSet,
    pub block_light_mask: BitSet,
    pub empty_sky_light_mask: BitSet,
    pub empty_block_light_mask: BitSet,
    pub sky_light_array_count: VarInt,
    pub sky_light_arrays: Vec<LightArray>,
    pub block_light_array_count: VarInt,
    pub block_light_arrays: Vec<LightArray>,
}

impl Light {
    pub fn new() -> Self {
        const SECTIONS: usize = 24;
        let mut sky_light_mask = BitSet::new(0);
        sky_light_mask.set_all();
        let mut block_light_mask = BitSet::new(0);
        block_light_mask.set_all();
        let empty_sky_light_mask = BitSet::new(0);
        let empty_block_light_mask = BitSet::new(0);

        // Create light arrays
        let mut sky_light_arrays = Vec::new();
        let mut block_light_arrays = Vec::new();

        Self {
            sky_light_mask: sky_light_mask.clone(),
            block_light_mask: sky_light_mask.clone(),
            empty_sky_light_mask: sky_light_mask.clone(),
            empty_block_light_mask: sky_light_mask.clone(),
            sky_light_array_count: VarInt(sky_light_arrays.len() as u64),
            sky_light_arrays,
            block_light_array_count: VarInt(block_light_arrays.len() as u64),
            block_light_arrays,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.extend_from_slice(&self.sky_light_mask.to_bytes());
        vec.extend_from_slice(&self.block_light_mask.to_bytes());
        vec.extend_from_slice(&self.empty_sky_light_mask.to_bytes());
        vec.extend_from_slice(&self.empty_block_light_mask.to_bytes());
        vec.extend_from_slice(&self.sky_light_array_count.write_varint());
        for i in &self.sky_light_arrays {
            vec.extend_from_slice(&i.data);
        }
        vec.extend_from_slice(&self.block_light_array_count.write_varint());
        for i in &self.block_light_arrays {
            vec.extend_from_slice(&i.data);
        }
        vec
    }
}

#[derive(Default, Debug, Clone)]
pub struct LightArray {
    pub data: Vec<u8>,
}
impl LightArray {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.data.clone()
    }
}
