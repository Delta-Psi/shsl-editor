pub mod formats;
pub mod game_data;

mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            InvalidString(std::string::FromUtf8Error);
            PngDecoding(png::DecodingError);
            Toml(toml::ser::Error);
        }

        errors {
            UnknownWadFile(path: String)
            UnknownWadDir(path: String)

            InvalidPakOffset
            InvalidPakIndices

            TgaDecoding
        }
    }
}

pub fn decode_utf16(data: &[u8]) -> String {
    let (string, _, _) = encoding_rs::UTF_16LE.decode(data);
    string.trim_end_matches('\0').to_string()
}

/// Allows rust sequential containers to be serialized and
/// deserialized as maps with keys as zero-padded indices.
///
/// The second field of this struct is the padding width.
pub struct Table<T>(pub T, pub usize);

impl<T> Table<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<I: serde::Serialize> serde::Serialize for Table<&[I]> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer,
    {
        use serde::ser::SerializeMap;

        let inner = &self.0;
        let mut map = serializer.serialize_map(Some(inner.len()))?;
        for (i, e) in inner.iter().enumerate() {
            map.serialize_entry(
                &format!("{:01$}", i, self.1),
                e,
            )?;
        }
        map.end()
    }
}

#[cfg(test)]
mod tests;
