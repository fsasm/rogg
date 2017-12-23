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

#[derive(Debug, PartialEq)]
pub enum GranulePosition {
    Position(u64),
    NoPosition,
}

impl From<u64> for GranulePosition {
    fn from(pos: u64) -> Self {
        if pos == 0xFFFFFFFFFFFFFFFFu64 {
            GranulePosition::NoPosition
        } else {
            GranulePosition::Position(pos)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate rand;
    use self::rand::distributions::{IndependentSample, Range};
    use self::rand::thread_rng;

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

    #[test]
    fn test_granule_position() {
        let range = Range::new(0, 0xFFFFFFFFFFFFFFFFu64);
        let mut rng = thread_rng();

        for i in 0..0xFFFF {
            let pos = range.ind_sample(&mut rng);
            assert_eq!(GranulePosition::Position(pos), GranulePosition::from(pos));
        }

        let i = 0xFFFFFFFFFFFFFFFFu64;
        assert_eq!(GranulePosition::NoPosition, GranulePosition::from(i));
    }
}
