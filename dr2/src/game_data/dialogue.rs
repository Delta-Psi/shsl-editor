use super::*;
use crate::formats::tga::{Tga, TgaExt};
use crate::formats::pak::Pak;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    // NAMES (text)
    {
        let mut buf = Vec::new();
        files.dr2_data_us.read_file("Dr2/data/us/bin/bin_progress_font_l.pak", &mut buf)?;
        let pak = Pak::from_bytes(&buf)?;
        let pak = Pak::from_bytes(&pak.entries[18])?;

        let mut names = toml::map::Map::with_capacity(pak.entries.len());
        for (i, entry) in pak.entries.iter().enumerate() {
            let name = crate::decode_utf16(entry);
            names.insert(format!("{:02}", i), toml::Value::String(name));
        }

        project.write_toml("dialogue/names.toml", &names)?;
    }

    // NAMES (images)
    {
        let mut buf = Vec::new();
        files.dr2_data_us.read_file("Dr2/data/us/cg/chara_name.pak", &mut buf)?;
        let pak = Pak::from_bytes(&buf)?;

        for (i, entry) in pak.entries.iter().enumerate() {
            let image = Tga::from_bytes(entry)?;
            let mut png = std::io::Cursor::new(Vec::new());
            image.to_png(&mut png)?;

            project.write_file(format!("dialogue/names/{:02}.png", i), &png.into_inner())?;
        }
    }

    // SPRITES
    {
        let wad = &files.dr2_data;

        for wad_path in wad.list_dir("Dr2/data/all/cg", true)? {
            // search for files matching `bustup_??_??.tga`
            let result = (|| {
                let string = wad_path.strip_prefix("Dr2/data/all/cg/bustup_")?;
                let string = string.strip_suffix(".tga")?;
                let indices: Vec<_> = string.splitn(2, '_').collect();

                Some((indices[0].parse::<u8>().ok()?, indices[1].parse::<u8>().ok()?))
            })();

            if let Some((character, sprite)) = result {
                let mut data = Vec::new();
                wad.read_file(&wad_path, &mut data)?;

                let image = Tga::from_bytes(&data)?;
                let mut png = std::io::Cursor::new(Vec::new());
                image.to_png(&mut png)?;

                project.write_file(format!("dialogue/sprites/{:02}/{:02}.png", character, sprite), &png.into_inner())?;
            }
        }
    }

    Ok(())
}

pub fn inject(files: &mut GameFiles, path: &Path) -> Result<()> {
    let _files = &files;
    let _path = &path;
    unimplemented!()
}
