use crate::varint::VarInt;

#[derive(Debug, Clone)]
pub struct ChunkSection {
    pub block_count: u16,
    pub palette: PaletteContainer,
    pub data_array_length: VarInt,
    pub data_array: Vec<u64>,
}

#[derive(Debug, Clone)]
pub struct PaletteContainer {
    pub bits_per_entry: u8,
    pub palette: PaletteFormats,
}

#[derive(Debug, Clone)]
pub enum PaletteFormats {
    SingleValued(VarInt),
    Indirect {
        palette_length: VarInt,
        palette: Vec<VarInt>,
    },
    Direct,
}

// impl ChunkSection {
//     pub fn new() -> Self {
//         Self { block_count: 100 }
//     }
// }
