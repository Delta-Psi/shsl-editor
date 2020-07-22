pub mod formats;
pub mod game_data;

mod errors {
    use error_chain::error_chain;

    error_chain! {
        foreign_links {
            Io(std::io::Error);
            InvalidString(std::string::FromUtf8Error);
            PngDecoding(png::DecodingError);
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

#[cfg(test)]
mod tests;
