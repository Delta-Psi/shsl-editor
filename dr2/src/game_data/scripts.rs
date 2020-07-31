use super::*;
use crate::formats::lin::Lin;

pub fn extract(project: &mut Project, files: &GameFiles) -> Result<()> {
    files.dr2_data_us.list_dir("Dr2/data/us/script", true)?
        .filter_map(|wad_path| {
            let string = wad_path.strip_prefix("Dr2/data/us/script/e")?;
            let string = string.strip_suffix(".lin")?;
            let indices: Vec<_> = string.splitn(3, '_').collect();

            let a: u8 = indices[0].parse().ok()?;
            let b: u16 = indices[1].parse().ok()?;
            let c: u16 = indices[2].parse().ok()?;

            Some((wad_path, a, b, c))
        }).try_for_each(|(wad_path, a, b, c)| {
            project.write_file(
                format!("scripts/{:02}/{:03}/{:03}.script", a, b, c),
                || {
                    let data = files.dr2_data_us.read_file(&wad_path)?;
                    let lin = Lin::from_bytes(&data)?;
                    let script = lin.to_script()?;

                    Ok(script.into_bytes())
                })
        })
}

pub fn inject(project: &mut Project, files: &mut GameFiles) -> Result<()> {
    files.dr2_data_us.list_dir("Dr2/data/us/script", true)?
        .filter_map(|wad_path| {
            let string = wad_path.strip_prefix("Dr2/data/us/script/e")?;
            let string = string.strip_suffix(".lin")?;
            let indices: Vec<_> = string.splitn(3, '_').collect();

            let a: u8 = indices[0].parse().ok()?;
            let b: u16 = indices[1].parse().ok()?;
            let c: u16 = indices[2].parse().ok()?;

            Some((wad_path, a, b, c))
        }).try_for_each(|(wad_path, a, b, c)| {
            project.open_file(
                format!("scripts/{:02}/{:03}/{:03}.script", a, b, c),
                |data| {
                    let string = std::str::from_utf8(data)
                        .chain_err(|| "script file is not valid UTF-8")?;
                    let lin = Lin::from_script(string)?.encode()?;
                    files.dr2_data_us.inject_file(&wad_path, &lin)?;

                    Ok(())
                }
            )?;

            Ok(())
        })
}
