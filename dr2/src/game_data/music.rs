use super::*;
use serde::Serialize;
use byteorder::{ByteOrder, LE};
use std::path::PathBuf;
use log::info;

pub const SAMPLE_RATE: f32 = 44100.0;
pub const TRACK_COUNT: usize = 102;

#[derive(Serialize)]
pub struct Track {
    pub path: PathBuf,
    pub loop_points: Option<(f32, f32)>,
}

pub fn extract(files: &GameFiles, path: &Path) -> Result<()> {
    let music_path = path.join("music");
    std::fs::create_dir_all(&music_path)?;
    let wad = &files.dr2_data.wad;

    let mut tracks = Vec::with_capacity(TRACK_COUNT);
    for index in 0..TRACK_COUNT {
        let ogg_path = music_path.join(format!("{:02}.ogg", index));
        let ogg_wad_path = format!("Dr2/data/all/bgm/dr2_bgm_hca.awb.{:05}.ogg", index);
        let loop_wad_path = format!("Dr2/data/all/bgm/dr2_bgm_hca.awb.{:05}.loop", index);

        let mut ogg_data = Vec::new();
        wad.read_file(&ogg_wad_path, &mut ogg_data)?;

        info!("writing {}", ogg_path.display());
        std::fs::write(&ogg_path, &ogg_data)?;

        let mut loop_points = None;
        if wad.files().contains_key(&loop_wad_path) {
            let mut loop_data = Vec::new();
            wad.read_file(&loop_wad_path, &mut loop_data)?;

            let begin = LE::read_u32(&loop_data[4..8]);
            let end = LE::read_u32(&loop_data[8..12]);

            loop_points = Some((begin as f32 / SAMPLE_RATE, end as f32 / SAMPLE_RATE));
        }

        tracks.push(Track {
            path: ogg_path.strip_prefix(path).unwrap().to_path_buf(),
            loop_points,
        });
    }

    {
        let path = path.join("music.toml");
        let tracks = toml::to_string_pretty(&crate::Table(tracks.as_ref(), 3))?;
        info!("writing {}", path.display());
        std::fs::write(path, tracks.as_bytes())?;
    }

    Ok(())
}
