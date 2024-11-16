use crate::varint::VarInt;
#[derive(Default, Clone, Debug, PartialEq, Eq)]
pub struct BitSet {
    data: Vec<u64>,
    size: usize,
}

impl BitSet {
    pub fn new(size: usize) -> Self {
        let num_blocks = (size + 63) / 64;
        BitSet {
            data: vec![0; num_blocks],
            size,
        }
    }

    pub fn empty() -> Self {
        BitSet {
            data: Vec::new(),
            size: 0,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] |= 1 << bit;
        }
    }

    pub fn clear(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] &= !(1 << bit);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            (self.data[block] & (1 << bit)) != 0
        } else {
            false
        }
    }

    pub fn toggle(&mut self, index: usize) {
        if index < self.size {
            let block = index / 64;
            let bit = index % 64;
            self.data[block] ^= 1 << bit;
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn count_ones(&self) -> usize {
        self.data
            .iter()
            .map(|&block| block.count_ones() as usize)
            .sum()
    }

    pub fn clear_all(&mut self) {
        self.data.fill(0);
    }

    pub fn set_all(&mut self) {
        self.data.fill(u64::MAX);
        // Clear any bits beyond the set size
        if self.size % 64 != 0 {
            let last_block = self.data.last_mut().unwrap();
            *last_block &= (1 << (self.size % 64)) - 1;
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        for value in &self.data {
            vec.extend_from_slice(&value.to_be_bytes());
        }
        let mut len = VarInt(self.size as u64).write_varint();
        len.append(&mut vec);
        len
    }
}
