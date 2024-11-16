#[derive(Default, Debug, Clone)]
pub struct SetBorderSize {
    diameter: f64,
}

impl SetBorderSize {
    const PROTOCOL_ID: u8 = 0x54;
    pub fn new(diameter: f64) -> Self {
        Self { diameter }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.extend_from_slice(&self.diameter.to_be_bytes());
        vec.insert(0, vec.len() as u8);
        vec
    }
}
