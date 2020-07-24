pub mod formats;
pub mod game_data;

pub mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            InvalidUtf8String(std::string::FromUtf8Error);
            InvalidUtf16String(std::string::FromUtf16Error);
            PngDecoding(png::DecodingError);
            PngEncoding(png::EncodingError);
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

pub fn decode_utf16(data: &[u8]) -> errors::Result<String> {
    use byteorder::{ByteOrder, LE};
    use error_chain::bail;

    if &data[0..2] != &[0xff, 0xfe] {
        bail!("couldn't find BOM");
    }
    let utf8 = &data[2..];

    let mut utf16 = vec![0; utf8.len() / 2];
    LE::read_u16_into(&utf8[0..utf16.len()*2], &mut utf16);

    // find zero terminator
    let i = match utf16.iter().enumerate().find(|(_, &v)| v == 0) {
        Some((i, _)) => i,
        None => bail!("string isn't zero terminated"),
    };
    let utf16 = &utf16[0..i];

    Ok(String::from_utf16(&utf16)?)
}

pub fn encode_utf16(string: &str) -> errors::Result<Vec<u8>> {
    use byteorder::{WriteBytesExt, LE};

    let mut buf = std::io::Cursor::new(Vec::new());
    buf.write_u8(0xff)?;
    buf.write_u8(0xfe)?;

    for v in string.encode_utf16() {
        buf.write_u16::<LE>(v)?;
    }

    buf.write_u16::<LE>(0)?;
    Ok(buf.into_inner())
}

#[cfg(test)]
mod tests;
