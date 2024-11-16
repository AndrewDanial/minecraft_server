use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum Property {
    Name(String),
    Value(String),
    IsSigned(bool),
    Signature(String),
}

#[derive(Debug, Clone, Default)]
pub struct LoginSuccess {
    pub uuid: Uuid,
    pub username: String, // len 16,
    pub num_of_properties: u8,
    pub properties: Vec<Property>,
    pub original_uuid_buffer: [u8; 16],
}

impl LoginSuccess {
    pub fn new(uuid: Uuid, original_uuid_buffer: [u8; 16], username: String) -> Self {
        LoginSuccess {
            uuid,
            username,
            num_of_properties: 0,
            properties: Vec::new(),
            original_uuid_buffer,
        }
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = vec![];
        let uuid_bytes = self.original_uuid_buffer; /*self.uuid.to_bytes_le();*/
        let username_len = self.username.len();
        let username_bytes = self.username.chars().map(|a| a as u8).collect::<Vec<u8>>();
        buffer.extend_from_slice(&uuid_bytes);
        buffer.push(username_len as u8);
        buffer.extend_from_slice(&username_bytes);
        buffer.push(self.num_of_properties);
        buffer
    }
}
