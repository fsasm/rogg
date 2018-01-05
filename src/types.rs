use super::crc32::*;

#[derive(Debug, PartialEq)]
pub struct Flags {
    continued: bool,
    first_page: bool,
    last_page: bool,
}

impl From<u8> for Flags {
    fn from(byte: u8) -> Self {
        Flags {
            continued: (byte & 0b0000_0001) != 0,
            first_page: (byte & 0b0000_0010) != 0,
            last_page: (byte & 0b0000_0100) != 0, // FIXME check if other unsupported flags are set
        }
    }
}

impl Flags {
    fn to_u8(&self) -> u8 {
        let mut res = 0u8;
        if self.continued {
            res |= 0x01u8;
        }
        if self.first_page {
            res |= 0x02u8;
        }
        if self.last_page {
            res |= 0x04u8;
        }
        res
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

impl GranulePosition {
    fn to_u64(&self) -> u64 {
        match self {
            &GranulePosition::NoPosition => 0xFFFFFFFFFFFFFFFFu64,
            &GranulePosition::Position(pos) => pos,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct OggHeader<'a> {
    pub flags: Flags,
    pub pos: GranulePosition,
    pub serial_number: u32,
    pub sequence_number: u32,
    pub crc32: u32,
    pub segment_table: &'a [u8],
}

impl<'a> OggHeader<'a> {
    pub fn get_data_size(&self) -> usize {
        self.segment_table
            .iter()
            .fold(0usize, |acc, &i| acc + (i as usize))
    }
}

#[derive(Debug, PartialEq)]
pub struct OggPage<'a> {
    pub header: OggHeader<'a>,
    pub data: &'a [u8],
}

impl<'a> OggPage<'a> {
    pub fn calc_crc(&self) -> u32 {
        let header = &self.header;
        let mut crc = Crc32::new();
        crc.process_u32(0x5367674Fu32);
        crc.process_u8(0);
        crc.process_u8(header.flags.to_u8());
        crc.process_u64(header.pos.to_u64());
        crc.process_u32(header.serial_number);
        crc.process_u32(header.sequence_number);
        crc.process_u32(0u32); // CRC is 0 during calculation
        crc.process_u8(header.segment_table.len() as u8);
        for &i in header.segment_table.iter() {
            crc.process_u8(i);
        }
        for &i in self.data.iter() {
            crc.process_u8(i);
        }

        crc.digest
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
            let reference = Flags {
                continued: (i & 0x01) != 0,
                first_page: (i & 0x02) != 0,
                last_page: (i & 0x04) != 0,
            };

            assert_eq!(reference, Flags::from(i));
        }
    }

    #[test]
    fn test_granule_position() {
        let range = Range::new(0, 0xFFFFFFFFFFFFFFFFu64);
        let mut rng = thread_rng();

        for _ in 0..0xFFFF {
            let pos = range.ind_sample(&mut rng);
            assert_eq!(GranulePosition::Position(pos), GranulePosition::from(pos));
        }

        let i = 0xFFFFFFFFFFFFFFFFu64;
        assert_eq!(GranulePosition::NoPosition, GranulePosition::from(i));
    }
}
