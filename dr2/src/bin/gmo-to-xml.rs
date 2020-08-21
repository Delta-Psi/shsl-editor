use byteorder::{ByteOrder, LE};
use xml::writer::{EventWriter, EmitterConfig, XmlEvent};
use std::io::prelude::*;

fn process_chunk(w: &mut EventWriter<impl Write>, data: &[u8]) -> Option<usize> {
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

    w.write(XmlEvent::start_element("chunk")
            .attr("type", &format!("{:04x}", type_))
            .attr("header", &hex::encode(header)))
        .unwrap();

    match match type_ {
        0x0002 => Some("file"),
        0x0003 => Some("subfile"),
        0x0004 => Some("bone info"),
        0x0005 => Some("model surface"),
        0x0006 => Some("mesh"),
        0x0007 => Some("vertex array"),
        0x0008 => Some("material"),
        0x0009 => Some("texture reference"),
        0x000a => Some("texture"),

        0x8066 => Some("mesh index data"),

        _ => None,
    } {
        Some(type_) => w.write(XmlEvent::comment(type_)).unwrap(),
        None => (),
    };

    let mut data = chunk_data;
    while !data.is_empty() {
        match process_chunk(w, data) {
            Some(chunk_size) => data = &data[chunk_size..],
            None => {
                w.write(XmlEvent::start_element("data")).unwrap();
                w.write(XmlEvent::characters(&hex::encode(data))).unwrap();
                w.write(XmlEvent::end_element()).unwrap();
                break
            },
        }
    }

    w.write(XmlEvent::end_element()).unwrap();

    Some(chunk_size)
}

fn gmo_to_xml(w: &mut EventWriter<impl Write>, data: &[u8]) {
    use dr2::formats::gmo::MAGIC_NUMBERS;
    assert_eq!(&data[0..16], MAGIC_NUMBERS);
    let data = &data[16..];

    process_chunk(w, data);
}

fn main() {
    let gmo_path = std::env::args().nth(1).unwrap();
    let data = std::fs::read(gmo_path).unwrap();

    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    let mut writer = EmitterConfig::new()
        .perform_indent(true)
        .create_writer(&mut stdout);

    gmo_to_xml(&mut writer, &data);
}
