use super::*;
use std::collections::BTreeMap;
use crate::formats::tga::{Tga, TgaExt};
use crate::formats::pak::Pak;
use serde::{Serialize, Deserialize};

const PRESENT_COUNT: usize = 140;

#[derive(Debug, Serialize, Deserialize)]
pub struct Present {
    pub name: String,
    pub description: String,
}

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    files.dr2_data.list_dir("Dr2/data/all/cg/present", true)?
        .filter_map(|wad_path| {
            let string = wad_path.strip_prefix("Dr2/data/all/cg/present/present_ico_")?;
            let string = string.strip_suffix(".tga")?;
            let index: u8 = string.parse().ok()?;

            Some((wad_path, index))
        }).filter(|(_, index)| *index < PRESENT_COUNT as u8)
        .try_for_each::<_, Result<_>>(|(wad_path, index)| {
            let data = files.dr2_data.read_file(&wad_path)?;
            let tga = Tga::from_bytes(&data)?;
            let png = tga.to_png()?;

            project.write_file(format!("presents/{:03}.png", index), &png)?;
            Ok(())
        })?;

    let pak = files.dr2_data_us.read_file("Dr2/data/us/bin/bin_progress_font_l.pak")?;
    let pak = Pak::from_bytes(&pak)?;
    let e2 = Pak::from_bytes(&pak.entries[2])?;
    let e3 = Pak::from_bytes(&pak.entries[3])?;

    
    let mut presents = BTreeMap::new();
    for index in 0..PRESENT_COUNT {
        use crate::decode_utf16;

        presents.insert(
            format!("{:03}", index),
            Present {
                name: decode_utf16(&e2.entries[index])?,
                description: decode_utf16(&e3.entries[index])?,
            },
        );
    }
    project.write_toml("presents.toml", &presents)?;

    Ok(())
}

pub fn inject(_project: &mut Project, _files: &mut GameFiles) -> Result<()> {
    Ok(())
}
