use super::*;
use std::collections::BTreeMap;
use serde::Serialize;
use byteorder::{ByteOrder, LE};

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
    let wad = &files.dr2_data.wad;

    let mut metadata = BTreeMap::new();

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
                println!("writing {}", path.display());
                std::fs::write(path, &data)?;
            } else {
                let begin = LE::read_u32(&data[4..8]);
                let end = LE::read_u32(&data[8..12]);

                metadata.insert(format!("{:02}", index), Track {
                    loop_begin: begin as f32 / SAMPLE_RATE,
                    loop_end: end as f32 / SAMPLE_RATE,
                });
            }
        }
    }

    {
        let path = path.join("music.toml");
        let metadata = toml::to_string_pretty(&metadata)?;
        println!("writing {}", path.display());
        std::fs::write(path, metadata.as_bytes())?;
    }

    Ok(())
}
