use crate::varint::VarInt;
#[derive(Debug, Clone)]
pub struct UpdateTags<'a> {
    pub length: VarInt,
    pub array_of_tags: Vec<TagArray<'a>>,
}

#[derive(Debug, Clone)]
pub struct TagArray<'a> {
    pub registry: &'a str,
    pub tag_array: Vec<Tag<'a>>,
}

#[derive(Debug, Clone)]
pub struct Tag<'a> {
    pub tag_name: &'a str,
    pub count: VarInt,
    pub entries: Vec<VarInt>,
}

impl<'a> UpdateTags<'a> {
    const PROTOCOL_ID: u8 = 0x0D;
    pub fn new(length: VarInt, array_of_tags: Vec<TagArray<'a>>) -> Self {
        Self {
            length,
            array_of_tags,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut vec = vec![];
        vec.push(Self::PROTOCOL_ID);
        vec.append(&mut self.length.write_varint());
        for tag_array in &self.array_of_tags {
            vec.push(tag_array.registry.len() as u8);
            vec.extend_from_slice(tag_array.registry.as_bytes());
            vec.push(tag_array.tag_array.len() as u8);
            for tag in &tag_array.tag_array {
                vec.push(tag.tag_name.len() as u8);
                vec.extend_from_slice(tag.tag_name.as_bytes());
                vec.append(&mut tag.count.write_varint());
                for entry in &tag.entries {
                    vec.append(&mut entry.write_varint());
                }
            }
        }
        vec.insert(0, vec.len() as u8);
        vec
    }
}
