#[test]
fn minimal_wad_header() {
    use std::io::{prelude::*, Cursor, SeekFrom};
    use byteorder::{WriteBytesExt, LittleEndian as LE};
    use crate::formats::wad;

    // construct such minimal header
    let mut data = Cursor::new(Vec::new());
    data.write(b"AGAR").unwrap(); // magic numbers
    data.write_u32::<LE>(1).unwrap(); // version
    data.write_u32::<LE>(2).unwrap();
    data.write_u32::<LE>(0).unwrap(); // unused value

    data.write_u32::<LE>(1).unwrap(); // file count
    data.write_u32::<LE>(4).unwrap(); // file path length
    data.write(b"file").unwrap(); // file path
    data.write_u64::<LE>(16).unwrap(); // file size
    data.write_u64::<LE>(0).unwrap(); // file offset

    data.write_u32::<LE>(1).unwrap(); // dir count (root)
    data.write_u32::<LE>(0).unwrap(); // dir path length (empty)
    data.write_u32::<LE>(1).unwrap(); // subfile count
    data.write_u32::<LE>(4).unwrap(); // subfile name length
    data.write(b"file").unwrap(); // subfile name
    data.write_u8(0).unwrap(); // subfile type

    data.seek(SeekFrom::Start(0)).unwrap();

    // check if the values match
    let header = wad::Header::read_from(data).unwrap();
    assert_eq!(header.version, [1, 2]);

    assert_eq!(header.files.len(), 1);
    assert_eq!(header.files[0].path, "file");
    assert_eq!(header.files[0].size, 16);
    assert_eq!(header.files[0].offset, 0);

    assert_eq!(header.dirs.len(), 1);
    assert_eq!(header.dirs[0].path, "");
    assert_eq!(header.dirs[0].subfiles.len(), 1);
    assert_eq!(header.dirs[0].subfiles[0].name, "file");
    assert_eq!(header.dirs[0].subfiles[0].is_directory, false);
}
