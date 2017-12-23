use types::*;

use nom::*;

named!(magic, tag!(&[0x4F, 0x67, 0x67, 0x53]));
named!(version, tag!([0x00]));
named!(
    header_flags<HeaderFlags>,
    map!(take!(1), |flags| HeaderFlags::from(flags[0]))
);
