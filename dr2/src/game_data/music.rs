use super::*;
use serde::Serialize;
use byteorder::{ByteOrder, LE};
use log::info;

pub const SAMPLE_RATE: f32 = 44100.0;
pub const TRACK_COUNT: usize = 102;

#[derive(Serialize)]
pub struct Track {
    pub loop_begin: f32,
    pub loop_end: f32,
}

pub fn extract(files: &GameFiles, path: &Path) -> Result<()> {
    let music_path = path.join("music");
    std::fs::create_dir_all(&music_path)?;
    let wad = &files.dr2_data;

    for wad_path in wad.list_dir("Dr2/data/all/bgm", true)? {
        let result = (|| {
            let string = wad_path.strip_prefix("Dr2/data/all/bgm/dr2_bgm_hca.awb.")?;
            if let Some(index) = string.strip_suffix(".ogg") {
                Some((index.parse::<u8>().ok()?, true))
            } else if let Some(index) = string.strip_suffix(".loop") {
                Some((index.parse::<u8>().ok()?, false))
            } else {
                None
            }
        })();

        if let Some((index, is_ogg)) = result {
            let mut data = Vec::new();
            wad.read_file(&wad_path, &mut data)?;

            if is_ogg {
                let path = music_path.join(format!("{:02}.ogg", index));
                info!("writing {}", path.display());
                std::fs::write(path, &data)?;
            } else {
                let loop_begin = LE::read_u32(&data[4..8]) as f32 / SAMPLE_RATE;
                let loop_end = LE::read_u32(&data[8..12]) as f32 / SAMPLE_RATE;

                let path = music_path.join(format!("{:02}.toml", index));
                info!("writing {}", path.display());
                std::fs::write(path, toml::to_string_pretty(&Track {
                    loop_begin,
                    loop_end,
                })?.as_bytes())?;
            }
        }
    }

    Ok(())
}
