use crate::varint::VarInt;

#[derive(Debug, Clone, Default)]
pub struct HandshakeData {
    pub protocol_version: i32,
    pub server_address: String,
    pub server_port: u16,
    pub next_state: u8,
}

impl HandshakeData {
    pub fn from_buffer(&mut self, buffer: &[u8]) {
        let protocol_type = buffer[0];
        let buffer_slice = &buffer[1..];
        let protocol_version = if let Ok((value, size)) = VarInt::read_varint(&buffer_slice) {
            value
        } else {
            0
        };
        let address_length = buffer_slice[2] as usize;
        let server_address = buffer_slice[3..3 + address_length]
            .iter()
            .map(|a| *a as char)
            .collect::<String>();
        let first = (buffer_slice[buffer_slice.len() - 3] as u16) << 8;
        let second = buffer_slice[buffer_slice.len() - 2];
        let server_port = first + second as u16;

        let next_state = buffer_slice[buffer_slice.len() - 1];

        self.protocol_version = protocol_version;
        self.server_address = server_address;
        self.server_port = server_port;
        self.next_state = next_state;
    }
}
