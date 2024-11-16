use crate::varint::VarInt;

#[derive(Debug, Default, Clone)]
pub struct SyncPlayerPos {
    pub teleport_id: VarInt,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub velocity_x: f64,
    pub velocity_y: f64,
    pub velocity_z: f64,
    pub yaw: f32,
    pub pitch: f32,
    pub flags: i32,
}

impl SyncPlayerPos {
    const PROTOCOL_ID: u8 = 0x42;
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.extend_from_slice(&self.teleport_id.write_varint());
        vec.extend_from_slice(&self.x.to_be_bytes());
        vec.extend_from_slice(&self.y.to_be_bytes());
        vec.extend_from_slice(&self.z.to_be_bytes());
        vec.extend_from_slice(&self.velocity_x.to_be_bytes());
        vec.extend_from_slice(&self.velocity_y.to_be_bytes());
        vec.extend_from_slice(&self.velocity_z.to_be_bytes());
        vec.extend_from_slice(&self.yaw.to_be_bytes());
        vec.extend_from_slice(&self.pitch.to_be_bytes());
        vec.extend_from_slice(&self.flags.to_be_bytes());
        vec.insert(0, vec.len() as u8);
        vec
    }
}
