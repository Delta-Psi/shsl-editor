use super::*;
use crate::formats::tga::{Tga, TgaExt};
use crate::formats::pak::Pak;

pub fn extract(files: &GameFiles, path: &Path) -> Result<()> {
    let path = path.join("dialogue");
    std::fs::create_dir_all(&path)?;

    // NAMES (text)
    {
        let path = path.join("names.toml");
        let mut buf = Vec::new();
        files.dr2_data_us.wad.read_file("Dr2/data/us/bin/bin_progress_font_l.pak", &mut buf)?;
        let pak = Pak::from_bytes(&buf)?;
        let pak = Pak::from_bytes(&pak.entries[18])?;

        let mut names = toml::map::Map::with_capacity(pak.entries.len());
        for (i, entry) in pak.entries.iter().enumerate() {
            let (name, _, _) = encoding_rs::UTF_16LE.decode(entry);
            names.insert(format!("{:02}", i), toml::Value::String(name.to_string()));
        }

        let file = toml::to_string_pretty(&names)?;
        println!("writing {}", path.display());
        std::fs::write(path, file.as_bytes())?;
    }

    // NAMES (images)
    {
        let path = path.join("names");
        std::fs::create_dir_all(&path)?;
        let mut buf = Vec::new();
        files.dr2_data_us.wad.read_file("Dr2/data/us/cg/chara_name.pak", &mut buf)?;
        let pak = Pak::from_bytes(&buf)?;

        for (i, entry) in pak.entries.iter().enumerate() {
            let image = Tga::from_bytes(entry)?;
            let mut png = std::io::Cursor::new(Vec::new());
            image.to_png(&mut png)?;

            let path = path.join(format!("{:02}.png", i));
            println!("writing {}", path.display());
            std::fs::write(path, &png.into_inner())?;
        }
    }

    // SPRITES
    {
        let path = path.join("sprites");
        let wad = &files.dr2_data.wad;

        for wad_path in wad.list_dir("Dr2/data/all/cg", true)? {
            // search for files matching `bustup_16_19.tga`
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

                let path = path.join(format!("{:02}", character));
                std::fs::create_dir_all(&path)?;
                let path = path.join(format!("{:02}.png", sprite));

                println!("writing {}", path.display());
                std::fs::write(path, &png.into_inner())?;
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
