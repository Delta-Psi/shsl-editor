use byteorder::{WriteBytesExt, LE};
use xml::reader::{EventReader, XmlEvent};
use xml::name::OwnedName;
use std::io::prelude::*;

fn process_chunk(r: &mut EventReader<impl Read>, type_: u16, header: &[u8]) -> Option<Vec<u8>> {
    let mut chunk_data = Vec::new();
    loop {
        match r.next().unwrap() {
            XmlEvent::EndDocument => panic!(),
            XmlEvent::EndElement {
                name,
            } if name.local_name == "chunk" => break,

            XmlEvent::StartElement {
                name,
                attributes,
                ..
            } => match name.local_name.as_ref() {
                "chunk" => {
                    assert_eq!(attributes.len(), 2);
                    assert_eq!(&attributes[0].name.local_name, "type");
                    assert_eq!(&attributes[1].name.local_name, "header");

                    let type_ = u16::from_str_radix(&attributes[0].value, 16).unwrap();
                    let header = hex::decode(&attributes[1].value).unwrap();

                    match process_chunk(r, type_, &header) {
                        Some(mut data) => chunk_data.append(&mut data),
                        None => break,
                    }
                },
                "data" => {
                    loop {
                        match r.next().unwrap() {
                            XmlEvent::EndElement {
                                name,
                            } if name.local_name == "data" => break,

                            XmlEvent::Characters(mut data) => {
                                data.retain(|c| !c.is_whitespace());
                                chunk_data.append(&mut hex::decode(&data).unwrap());
                            },

                            _ => (),
                        }
                    }
                },

                _ => panic!(),
            },

            _ => (),
        }
    }

    let mut data = Vec::new();
    data.write_u16::<LE>(type_).unwrap();
    let data_offset = match header.len() {
        0 => 0,
        len => 8 + len as u16,
    };
    data.write_u16::<LE>(data_offset).unwrap();
    data.write_u32::<LE>(8 + header.len() as u32 + chunk_data.len() as u32).unwrap();

    data.write_all(header).unwrap();
    data.append(&mut chunk_data);

    Some(data)
}

fn main() {
    let stdin = std::io::stdin();
    let stdin = stdin.lock();
    let mut reader = EventReader::new(stdin);

    let gmo_path = std::env::args().nth(1).unwrap();
    let mut writer = std::fs::File::create(gmo_path).unwrap();

    use dr2::formats::gmo::MAGIC_NUMBERS;
    writer.write_all(MAGIC_NUMBERS).unwrap();

    loop {
        match reader.next().unwrap() {
            XmlEvent::EndDocument => break,

            XmlEvent::StartElement {
                name: OwnedName {
                    local_name,
                    ..
                },
                attributes,
                ..
            } if local_name == "chunk" => {
                assert_eq!(attributes.len(), 2);
                assert_eq!(&attributes[0].name.local_name, "type");
                assert_eq!(&attributes[1].name.local_name, "header");

                let type_ = u16::from_str_radix(&attributes[0].value, 16).unwrap();
                let header = hex::decode(&attributes[1].value).unwrap();

                match process_chunk(&mut reader, type_, &header) {
                    Some(data) => writer.write_all(&data).unwrap(),
                    None => break,
                }
            },

            _ => (),
        }
    }
}
