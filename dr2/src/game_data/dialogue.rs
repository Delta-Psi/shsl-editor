use super::*;
use std::collections::BTreeMap;
use std::borrow::Cow;
use crate::formats::tga::{Tga, TgaExt};
use crate::formats::pak::Pak;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    // NAMES (text)
    let pak = files.dr2_data_us.read_file("Dr2/data/us/bin/bin_progress_font_l.pak")?;
    let pak = Pak::from_bytes(&pak)?;
    let e18 = Pak::from_bytes(&pak.entries[18])?;

    let mut names = BTreeMap::new();
    for (i, entry) in e18.entries.iter().enumerate() {
        let name = crate::decode_utf16(entry)?;
        names.insert(format!("{:02}", i), name);
    }

    project.write_toml("dialogue/names.toml", &names)?;

    // NAMES (images)
    let pak = files.dr2_data_us.read_file("Dr2/data/us/cg/chara_name.pak")?;
    let pak = Pak::from_bytes(&pak)?;

    for (i, entry) in pak.entries.iter().enumerate() {
        let image = Tga::from_bytes(entry)?;
        let png = image.to_png()?;

        project.write_file(format!("dialogue/names/{:02}.png", i), &png)?;
    }

    // SPRITES
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
            let tga = wad.read_file(&wad_path)?;
            let image = Tga::from_bytes(&tga)?;
            let png = image.to_png()?;

            project.write_file(format!("dialogue/sprites/{:02}/{:02}.png", character, sprite), &png)?;
        }
    }

    Ok(())
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    // NAMES (text)
    project.open_file("dialogue/names.toml", |data| {
        let pak = files.dr2_data_us.read_file("Dr2/data/us/bin/bin_progress_font_l.pak")?;
        let mut pak = Pak::from_bytes(&pak)?;
        let mut e18 = Pak::from_bytes(&pak.entries[18])?;

        let names: BTreeMap<String, String> = toml::de::from_slice(&data)?;
        for (i, entry) in e18.entries.iter_mut().enumerate() {
            let name = &names[&format!("{:02}", i)];
            let name = crate::encode_utf16(name)?;
            *entry = Cow::Owned(name);
        }

        let e18 = e18.repack()?;
        pak.entries[18] = Cow::Owned(e18);
        let pak = pak.repack()?;

        files.dr2_data_us.inject_file("Dr2/data/us/bin/bin_progress_font_l.pak", &pak)?;

        Ok(())
    })?;

    // NAMES (images)
    let pak = files.dr2_data_us.read_file("Dr2/data/us/cg/chara_name.pak")?;
    let mut pak = Pak::from_bytes(&pak)?;
    let mut modified = false;

    for (i, entry) in pak.entries.iter_mut().enumerate() {
        project.open_file(&format!("dialogue/names/{:02}.png", i), |data| {
            let tga = Tga::from_png(data)?;

            *entry = Cow::Owned(tga);

            modified = true;
            Ok(())
        })?;
    }

    if modified {
        let pak = pak.repack()?;
        files.dr2_data_us.inject_file("Dr2/data/us/cg/chara_name.pak", &pak)?;
    }

    // SPRITES
    for wad_path in files.dr2_data.list_dir("Dr2/data/all/cg", true)? {
        // search for files matching `bustup_??_??.tga`
        let result = (|| {
            let string = wad_path.strip_prefix("Dr2/data/all/cg/bustup_")?;
            let string = string.strip_suffix(".tga")?;
            let indices: Vec<_> = string.splitn(2, '_').collect();

            Some((indices[0].parse::<u8>().ok()?, indices[1].parse::<u8>().ok()?))
        })();

        if let Some((character, sprite)) = result {
            project.open_file(format!("dialogue/sprites/{:02}/{:02}.png", character, sprite), |data| {
                let tga = Tga::from_png(&data)?;

                files.dr2_data.inject_file(&wad_path, &tga)?;

                Ok(())
            })?;
        }
    }

    Ok(())
}
