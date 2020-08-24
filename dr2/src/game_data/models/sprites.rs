use super::*;
use crate::formats::tga::{Tga, TgaExt};

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    // SPRITE MODELS
    for wad_path in files.dr2_data.list_dir("Dr2/data/all/model", true)? {
        // match stand_*.gmo, and then replace with stand_*_s.gmo if it exists
        let result = (|| {
            let string = wad_path.strip_prefix("Dr2/data/all/model/stand_")?;
            let string = string.strip_suffix(".gmo")?;
            let indices: Vec<_> = string.splitn(2, '_').collect();

            Some((
                    indices[0].parse::<u8>().ok()?,
                    indices[1].parse::<u8>().ok()?,
            ))
        })();

        if let Some((character, sprite)) = result {
            let data = if let Ok(data) = files.dr2_data.read_file(&format!("Dr2/data/all/model/stand_{:02}_{:02}_s.gmo", character, sprite)) {
                data
            } else {
                files.dr2_data.read_file(&wad_path)?
            };

            project.write_file(
                format!("models/sprites/{:02}/{:02}/model.gmo", character, sprite),
                || Ok(data),
            )?;
        }
    }

    // SPRITE TEXTURES
    // just dumps the entire folder
    for wad_path in files.dr2_data.list_dir("Dr2/data/all/texture", true)? {
        // match stand_*.tga
        let result = (|| {
            let string = wad_path.strip_prefix("Dr2/data/all/texture/stand_")?;
            let string = string.strip_suffix(".tga")?;

            Some(string)
        })();

        if let Some(string) = result {
            project.write_file(
                format!("models/sprites/textures/{}.png", string),
                || {
                    let tga = files.dr2_data.read_file(&wad_path)?;
                    let image = Tga::from_bytes(&tga)?;
                    let png = image.to_png()?;

                    Ok(png)
                })?;
        }
    }

    Ok(())
}
