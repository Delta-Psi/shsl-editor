use dr2::formats::gmo::MAGIC_NUMBERS;
use byteorder::{ByteOrder, LE};

const INDENT: &str = "  ";

fn process_chunk(data: &[u8], indent_level: usize) -> Option<usize> {
    if data.len() < 8 {
        return None;
    }

    let type_ = LE::read_u16(&data[0..2]);
    let data_offset = match LE::read_u16(&data[2..4]) {
        0 => 8,
        offset => offset as usize,
    };
    let chunk_size = LE::read_u32(&data[4..8]) as usize;

    if chunk_size < data_offset || data.len() < chunk_size {
        return None;
    }

    let header = &data[8 .. data_offset];
    let chunk_data = &data[data_offset .. chunk_size];

    let indent = INDENT.repeat(indent_level);
    print!("{}chunk {:04x} [{}] {{", indent, type_, hex::encode(header));
    match type_ {
        0x0002 => print!(" # file"),
        0x0003 => print!(" # subfile"),
        0x0004 => print!(" # bone info"),
        0x0005 => print!(" # model surface"),
        0x0006 => print!(" # mesh"),
        0x0007 => print!(" # vertex array"),
        0x0008 => print!(" # material"),
        0x0009 => print!(" # texture reference"),
        0x000a => print!(" # texture"),

        0x8066 => print!(" # mesh index data"),


        _ => (),
    };
    println!();

    if process_subchunks(chunk_data, indent_level+1).is_none() {
        let indent = INDENT.repeat(indent_level+1);
        let mut data = chunk_data;
        while !data.is_empty() {
            if data.len() >= 16 {
                println!("{}{}", indent, hex::encode(&data[0..16]));
                data = &data[16..];
            } else {
                println!("{}{}", indent, hex::encode(data));
                break;
            }
        }
    }

    println!("{}}}", indent);

    Some(chunk_size)
}

fn process_subchunks(data: &[u8], indent_level: usize) -> Option<()> {
    let mut data = data;

    while !data.is_empty() {
        let size = process_chunk(data, indent_level)?;
        data = &data[size..];
    }

    Some(())
}

fn main() {
    let gmo_path = std::env::args().nth(1).unwrap();
    let data = std::fs::read(gmo_path).unwrap();

    assert_eq!(&data[0..16], MAGIC_NUMBERS);
    let data = &data[16..];

    process_subchunks(data, 0);
}
