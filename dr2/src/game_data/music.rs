use super::*;
use serde::Serialize;
use byteorder::{ByteOrder, LE};

pub const SAMPLE_RATE: f32 = 44100.0;
pub const TRACK_COUNT: usize = 102;

#[derive(Serialize, Deserialize)]
pub struct Track {
    pub loop_begin: f32,
    pub loop_end: f32,
}

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
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
            let data = wad.read_file(&wad_path)?;

            if is_ogg {
                project.write_file(format!("music/{:03}.ogg", index), &data)?;
            } else {
                let loop_begin = LE::read_u32(&data[4..8]) as f32 / SAMPLE_RATE;
                let loop_end = LE::read_u32(&data[8..12]) as f32 / SAMPLE_RATE;

                project.write_toml(format!("music/{:03}.toml", index), &Track {
                    loop_begin,
                    loop_end,
                })?;
            }
        }
    }

    Ok(())
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    let wad = &mut files.dr2_data;

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
            if is_ogg {
                project.open_file(format!("music/{:03}.ogg", index), |data| {
                    wad.inject_file(&wad_path, data)?;

                    Ok(())
                })?;
            } else {
                project.open_file(format!("music/{:03}.toml", index), |data| {
                    let track: Track = toml::de::from_slice(&data)?;

                    let loop_begin = (track.loop_begin * SAMPLE_RATE) as u32;
                    let loop_end = (track.loop_end * SAMPLE_RATE) as u32;

                    let mut data = wad.read_file(&wad_path)?;

                    LE::write_u32(&mut data[4..8], loop_begin);
                    LE::write_u32(&mut data[8..12], loop_end);

                    wad.inject_file(&wad_path, &data)?;

                    Ok(())
                })?;
            }
        }
    }

    Ok(())
}
