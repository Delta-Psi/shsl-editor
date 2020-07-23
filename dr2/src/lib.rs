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

#[cfg(test)]
mod tests;
