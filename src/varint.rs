#[derive(Default, Debug, Clone, Copy)]
pub struct VarInt(pub u64);

impl VarInt {
    const SEGMENT_BITS: u64 = 0x7F;
    const CONTINUE_BIT: u64 = 0x80;
    pub fn read_varint(byte: &[u8]) -> Result<(i32, usize), String> {
        let mut value = 0;
        let mut position = 0;
        let mut index = 0;
        loop {
            value |= ((byte[index] as u64 & Self::SEGMENT_BITS) as i32) << position;

            if (byte[index] as u64 & Self::CONTINUE_BIT) == 0 {
                break;
            }

            position += 7;
            index += 1;

            if position >= 32 {
                return Err(String::from("Bad VarInt"));
            }
        }

        return Ok((value, index + 1));
    }

    pub fn write_varint(&self) -> Vec<u8> {
        let mut bytes = vec![];
        let mut data = self.0;
        loop {
            if (data & !(Self::SEGMENT_BITS)) == 0 {
                bytes.push(data as u8);
                return bytes;
            }

            bytes.push(((data & Self::SEGMENT_BITS) | Self::CONTINUE_BIT) as u8);
            data >>= 7;
        }
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_write_varint() {
        assert_eq!(vec![221, 199, 1], VarInt(25565).write_varint());
    }

    #[test]
    fn test_write_varint_0() {
        assert_eq!(vec![0], VarInt(0).write_varint());
    }

    #[test]
    fn test_write_varint_1() {
        assert_eq!(vec![1], VarInt(1).write_varint());
    }

    #[test]
    fn test_write_varint_2048() {
        assert_eq!(vec![128, 16], VarInt(2048).write_varint());
    }

    #[test]
    fn read_var_int() {
        assert_eq!(VarInt::read_varint(&[221, 199, 1]), Ok((25565, 3)));
    }
}
