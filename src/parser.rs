use types::*;
use nom::*;

named!(magic, tag!(&[0x4F, 0x67, 0x67, 0x53]));
named!(version, tag!([0x00]));
named!(
    header_flags<Flags>,
    map!(take!(1), |flags| Flags::from(flags[0]))
);

named!(
    granule_position<GranulePosition>,
    map!(le_u64, |pos| GranulePosition::from(pos))
);

named!(serial_number<u32>, do_parse!(serial: le_u32 >> (serial)));
named!(sequence_number<u32>, do_parse!(seq: le_u32 >> (seq)));
named!(crc<u32>, do_parse!(crc: le_u32 >> (crc)));
named!(segment_table<&[u8]>, length_data!(le_u8));

#[cfg_attr(rustfmt, rustfmt_skip)]
named!(
    pub parse_header<OggHeader>,
    do_parse!(
        magic >>
        version >>
        flags: header_flags >>
        pos: granule_position >>
        serial: serial_number >>
        seq_number: sequence_number >>
        crc32: crc >>
        seg_table: segment_table >>
        (OggHeader {
            flags: flags,
            pos: pos,
            serial_number: serial,
            sequence_number: seq_number,
            crc32: crc32,
            segment_table: seg_table,
        })
    )
);

named!(
    pub parse_page<OggPage>,
    do_parse!(
        header: parse_header >>
        data: take!(header.get_data_size()) >>
        (OggPage {
            header: header,
            data: data,
        })
    )
);

named!(parse_all_pages<Vec<OggPage> >, many0!(parse_page));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_file() {
        let data = include_bytes!("../test/test01.ogg");
        assert!(parse_header(data).is_done());
        assert!(parse_page(data).is_done());
    }

    #[test]
    fn test_crc() {
        let data = include_bytes!("../test/test02.ogg");
        let (_, page) = parse_page(data).unwrap();
        let digest = page.calc_crc();
        assert_eq!(digest, page.header.crc32);
    }

    #[test]
    fn test_crc_wikipedia() {
        let data = include_bytes!("../test/Example.ogg");
        let (rem, pages) = parse_all_pages(data).unwrap();

        assert_eq!(rem.len(), 0);

        for page in pages.iter() {
            let digest = page.calc_crc();
            assert_eq!(digest, page.header.crc32);
        }
    }
}
