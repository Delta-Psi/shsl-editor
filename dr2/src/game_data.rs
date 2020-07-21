use crate::formats::wad::Wad;

use error_chain::error_chain;
error_chain! {
    foreign_links {
        Io(std::io::Error);
    }
}

/// Contains handles to every relevant game file.
pub struct GameFiles {
    pub dr2_data: Wad,
    pub dr2_data_us: Wad,
}

pub trait GameData: Sized {
    fn extract(files: &GameFiles) -> Result<Self>;
    fn inject(&self, files: &mut GameFiles) -> Result<()>;
}

pub mod music;
