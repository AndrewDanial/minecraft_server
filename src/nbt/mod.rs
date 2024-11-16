#[repr(u8)]
#[derive(Default, Debug, Clone)]
pub enum NBT {
    #[default]
    TagEnd = 0,
    TagByte(&'static str, u8) = 1,
    TagShort(&'static str, u16) = 2,
    TagInt(&'static str, u32) = 3,
    TagLong(&'static str, u64) = 4,
    TagFloat(&'static str, f32) = 5,
    TagDouble(&'static str, f64) = 6,
    TagByteArray(&'static str, Vec<u8>) = 7,
    TagString(&'static str, &'static str) = 8,
    TagList(&'static str, Vec<NBT>) = 9,
    TagCompound(&'static str, Vec<NBT>) = 10,
    TagIntArray(&'static str, Vec<i32>) = 11,
    TagLongArray(&'static str, Vec<i64>) = 12,
}

impl NBT {
    pub fn tag_to_bytes(tag: NBT) -> Vec<u8> {
        let mut vec = vec![];
        match tag {
            NBT::TagEnd => {
                vec.push(0);
            }
            NBT::TagByte(name, value) => {
                vec.push(1);
                let len = name.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                if name.len() > 0 {
                    vec.extend_from_slice(name.as_bytes());
                }
                vec.push(value)
            }
            NBT::TagShort(name, value) => {
                vec.push(2);
                let len = name.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                if name.len() > 0 {
                    vec.extend_from_slice(name.as_bytes());
                }
                let top = ((value & 0xFF00) >> 8) as u8;
                let bottom = (value & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
            }
            NBT::TagInt(name, value) => {
                vec.push(3);
                let len = name.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                if name.len() > 0 {
                    vec.extend_from_slice(name.as_bytes());
                }
                let top = ((value & 0xFF000000) >> 24) as u8;
                let middle = ((value & 0x00FF00000) >> 16) as u8;
                let middle2 = ((value & 0x0000FF00) >> 8) as u8;
                let bottom = (value & 0x000000FF) as u8;
                vec.push(top);
                vec.push(middle);
                vec.push(middle2);
                vec.push(bottom);
            }
            NBT::TagFloat(name, value) => {
                vec.push(4);
                let len = name.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                if name.len() > 0 {
                    vec.extend_from_slice(name.as_bytes());
                }

                let val = value.to_be_bytes();
                vec.push(val[0]);
                vec.push(val[1]);
                vec.push(val[2]);
                vec.push(val[3]);
            }
            NBT::TagString(name, value) => {
                vec.push(8);
                let len = name.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                if name.len() > 0 {
                    vec.extend_from_slice(name.as_bytes());
                }
                let len = value.len() as u16;
                let top = ((len & 0xFF00) >> 8) as u8;
                let bottom = (len & 0x00FF) as u8;
                vec.push(top);
                vec.push(bottom);
                vec.extend_from_slice(value.as_bytes());
            }
            NBT::TagCompound(_, entries) => {
                vec.push(10);
                for tag in entries {
                    vec.append(&mut Self::tag_to_bytes(tag));
                }
                vec.append(&mut NBT::tag_to_bytes(NBT::TagEnd));
            }

            // TODO: Float, Double, ByteArray, List, Int Array, Long Array
            unhandled => {
                println!("Entry '{:?}' not handled", unhandled);
            }
        }
        vec
    }
}

mod tests {
    use crate::nbt::NBT;

    #[test]
    fn test_string() {
        assert_eq!(
            vec![
                0x08, // Tag ID for string
                0x00, 0x0A, // Name length (10)
                0x65, 0x78, 0x61, 0x6D, 0x70, 0x6C, 0x65, 0x54, 0x61, 0x67, // "exampleTag"
                0x00, 0x11, // String length (16)
                0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x2C, 0x20, 0x4D, 0x69, 0x6E, 0x65, 0x63, 0x72,
                0x61, // "Hello, Minecraft!"
                0x66, 0x74, 0x21
            ],
            NBT::tag_to_bytes(NBT::TagString("exampleTag", "Hello, Minecraft!"))
        );
    }
}
