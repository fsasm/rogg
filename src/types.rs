#[derive(Debug, PartialEq)]
pub struct HeaderFlags {
    continued: bool,
    first_page: bool,
    last_page: bool,
}

impl From<u8> for HeaderFlags {
    fn from(byte: u8) -> Self {
        HeaderFlags {
            continued: (byte & 0b0000_0001) != 0,
            first_page: (byte & 0b0000_0010) != 0,
            last_page: (byte & 0b0000_0100) != 0, // FIXME check if other unsupported flags are set
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header_flags() {
        for i in 0..255u8 {
            let reference = HeaderFlags {
                continued: (i & 0x01) != 0,
                first_page: (i & 0x02) != 0,
                last_page: (i & 0x04) != 0,
            };

            assert_eq!(reference, HeaderFlags::from(i));
        }
    }
}
