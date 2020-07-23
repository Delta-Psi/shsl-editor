pub mod formats;
pub mod game_data;

mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            InvalidString(std::string::FromUtf8Error);
            PngDecoding(png::DecodingError);
            TomlSerialize(toml::ser::Error);
            TomlDeserialize(toml::de::Error);
        }

        errors {
            UnknownWadFile(path: String)
            UnknownWadDir(path: String)

            InvalidPakOffset
            InvalidPakIndices

            TgaDecoding

            Utf16Encoding
        }
    }
}

pub fn decode_utf16(data: &[u8]) -> String {
    let (string, _, _) = encoding_rs::UTF_16LE.decode(data);
    string.trim_end_matches('\0').to_string()
}

pub fn encode_utf16(string: &str) -> Vec<u8> {
    use byteorder::{WriteBytesExt, LE};

    let mut buf = std::io::Cursor::new(Vec::new());
    buf.write_u8(0xff).unwrap();
    buf.write_u8(0xfe).unwrap();

    for v in string.encode_utf16() {
        buf.write_u16::<LE>(v).unwrap();
    }

    buf.write_u16::<LE>(0).unwrap();
    buf.into_inner()
}

#[cfg(test)]
mod tests;
