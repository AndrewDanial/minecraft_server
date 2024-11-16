use uuid::Uuid;

#[derive(Debug, Clone, Default)]
pub struct LoginStart {
    pub username: String,
    pub uuid: Uuid,
    pub original_uuid_buffer: [u8; 16],
}

impl LoginStart {
    pub fn from_buffer(&mut self, buffer: &[u8]) {
        let buffer_len = buffer[0] as usize;
        if buffer_len == 1 {
            return;
        }
        let buffer_slice = &buffer[2..buffer_len + 1];
        let username_len = buffer_slice[0] as usize;
        let username = buffer_slice[1..username_len + 1]
            .iter()
            .map(|a| *a as char)
            .collect::<String>();

        let mut uuid_buffer = [0u8; 16];
        let uuid_slice = &buffer_slice[9..];
        for i in 0..uuid_slice.len() {
            uuid_buffer[i] = uuid_slice[i];
        }
        self.username = username;
        self.uuid = Uuid::from_bytes(uuid_buffer);
        self.original_uuid_buffer = uuid_buffer
    }
}
