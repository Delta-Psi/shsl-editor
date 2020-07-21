use super::*;

pub const CHARACTER_COUNT: usize = 25;

pub struct Sprites;

impl Data for Sprites {
    fn extract<P: AsRef<Path>>(files: &GameFiles, path: P) -> Result<()> {
        use std::io::prelude::*;
        let path = path.as_ref();
        let wad = &files.dr2_data.wad;

        for wad_path in wad.list_dir("Dr2/data/all/cg", true)? {
            // search for files matching `bustup_16_19.tga`
            let result = (|| {
                let string = wad_path.strip_prefix("Dr2/data/all/cg/bustup_")?;
                let string = string.strip_suffix(".tga")?;
                let indices: Vec<_> = string.splitn(2, "_").collect();

                Some((indices[0].parse::<u8>().ok()?, indices[1].parse::<u8>().ok()?))
            })();
            
            if let Some((character, sprite)) = result {
                let mut data = Vec::new();
                wad.read_file(&wad_path, &mut data)?;

                let path = path.join(format!("{:02}", character));
                std::fs::create_dir_all(&path)?;
                let path = path.join(format!("{:02}.tga", sprite));

                println!("writing {}", path.display());
                let mut file = std::fs::File::create(path)?;
                file.write_all(&data)?;
            }
        }

        Ok(())
    }
}
